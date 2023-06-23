use std::{future::IntoFuture, marker::PhantomData};

/// This Trait is mainly responsible for converting `ViewModel` to `Dto`.
///
/// Originally, the Application Layer (a.k.a. UseCase Layer) is responsible
/// for this function, but I decided that there is no problem here either.
///
/// In addition, role of this is almost the same as that of [`OutputPort`],
/// but it is separated so that it can be easily searched in the editor.
///
/// ### Type Argument
/// + `I`   - ViewModel.
/// + `Dto` - Data Transfer Object. See [`application::transfer`]
///
///   `It is expected to be the argument of the Service defined in the UseCase layer.`
///
/// ## Usage
/// ```rust
/// #
/// # use server::controller::InputPort;
/// #
/// # pub struct ViewModel {
/// #     text: String
/// # }
/// #
/// # pub struct TextInputDto {
/// #     pub text: String
/// # }
/// #
///
/// pub struct Transformer;
///
/// impl InputPort<ViewModel> for Transformer {
///     type Dto = TextInputDto;
///     fn emit(&self, input: ViewModel) -> Self::Dto {
///         TextInputDto {
///             text: input.text
///         }
///     }
/// }
///
pub trait InputPort<I>: 'static + Sync + Send {
    type Dto;
    fn emit(&self, input: I) -> Self::Dto;
}


/// This Trait is mainly responsible for converting the `Dto` output
/// from the Application Layer (a.k.a UseCase Layer) Service into a `ViewModel`. `(UseCaseOutput -> ViewModel)`
///
/// In addition, role of this is almost the same as that of InputPort,
/// but it is separated, so that it can be easily searched in the editor.
///
/// ### Type Argument
/// + `I` - Data Transfer Object.
///
///   `Note: The type here specifies the return value of the Service defined in the Application layer.`
///
///         `The image is "Service -> Value -> OutputPort -> ViewModel".`
///
/// + `ViewModel` - ViewModel.
///
/// ## Usage
/// ```rust
/// #
/// # extern crate thiserror;
/// # use server::controller::OutputPort;
/// #
/// # pub struct OutputDto {
/// #     pub id: String,
/// #     pub name: String,
/// #     pub text: String
/// # }
/// #
/// # pub struct ViewModel {
/// #     name: String,
/// #     text: String
/// # }
/// #
/// # #[derive(Debug, thiserror::Error)]
/// # pub enum UseCaseError {
/// #     #[error("failed task in usecase")]
/// #     Process
/// # }
/// #
/// # #[derive(Debug, thiserror::Error)]
/// # pub enum PresentationError {
/// #     #[error("{0}")]
/// #     UseCase(&'static str)
/// # }
///
/// pub struct Presenter;
///
/// impl OutputPort<Result<OutputDto, UseCaseError>> for Presenter {
///     type ViewModel = Result<ViewModel, PresentationError>;
///     fn emit(&self, input: Result<OutputDto, UseCaseError>) -> Self::ViewModel {
///         input.map(|dto| ViewModel {
///             name: dto.name,
///             text: dto.text
///         })
///         .map_err(|_| {
///             PresentationError::UseCase("error at usecase layer.")
///         })
///     }
/// }
///
pub trait OutputPort<I>: 'static + Sync + Send {
    type ViewModel;
    fn emit(&self, input: I) -> Self::ViewModel;
}


/// This is a Controller defined in the Clean Architecture.
///
/// ### Note
/// that the Presenter and Input Boundary are implemented declarative
/// in order to use the services defined in the UseCase layer in a general purpose.
///
/// This is to comply with the restrictions of the language specification,
/// and to make it easier to support module-oriented frameworks
/// such as `Axum` by completing them in the Presentation layer.
///
/// ## Usage
/// + See [Test Code](tests)
///
pub struct Controller<T, P, I, D, O> {
    transformer: T,
    presenter: P,
    _i: PhantomData<I>,
    _t: PhantomData<D>,
    _o: PhantomData<O>
}

impl<T, P, I, D, O> Controller<T, P, I, D, O>
    where T: InputPort<I, Dto = D>,
          P: OutputPort<O>
{
    /// Initialize controller.
    ///
    /// ## Arguments
    /// + `transformer` - Struct implemented [`InputPort`].
    /// + `presenter`   - Struct implemented [`OutputPort`].
    ///
    pub fn new(transformer: T, presenter: P) -> Self {
        Self { transformer, presenter, _i: PhantomData, _t: PhantomData, _o: PhantomData }
    }

    /// Receive ViewModel value.
    ///
    /// ## Argument
    /// + `input` - Value of ViewModel.
    pub fn transform(self, input: I) -> Transformed<T, P, I, D, O> {
        Transformed { trans: self.transformer.emit(input), controller: self, _i: PhantomData, _o: PhantomData }
    }

    fn present(self) -> P {
        self.presenter
    }
}

pub struct Transformed<T, P, I, D, O> {
    trans: D,
    controller: Controller<T, P, I, D, O>,
    _i: PhantomData<I>,
    _o: PhantomData<O>
}

impl<T, P, I, D, O> Transformed<T, P, I, D, O>
    where T: InputPort<I, Dto = D>,
          P: OutputPort<O>
{
    /// The value received by [`Controller::transform`] is transformed based
    /// on the argument `transformer` received during [`Controller::new`],
    /// and a closure is provided with it as an argument.
    ///
    /// Note: Return value is a converted value in accordance with the presenter specified at [`Controller::new`].
    ///
    /// Image as `ViewModel -> Controller::transform -> |Dto| { closure } -> presenter -> ViewModel`
    pub async fn handle<F, Fut>(self, f: F) -> P::ViewModel
        where F: FnOnce(D) -> Fut,
              Fut: IntoFuture<Output = O>
    {
        self.controller.present().emit(f(self.trans).await)
    }
}



#[cfg(test)]
mod tests {
    use super::{Controller, InputPort, OutputPort};

    pub struct RequestForm {
        name: String
    }

    pub struct TransformerA;

    impl InputPort<RequestForm> for TransformerA {
        type Dto = DataDto;
        fn emit(&self, input: RequestForm) -> Self::Dto {
            DataDto("abc123".to_string(), input.name)
        }
    }

    pub struct DataDto(pub String, pub String);

    #[allow(unused)]
    #[derive(Debug, serde::Serialize)]
    pub struct PresentationalDataA {
        id: String,
        name: String
    }

    pub struct PresenterA;

    impl OutputPort<anyhow::Result<DataDto>> for PresenterA {
        type ViewModel = anyhow::Result<PresentationalDataA>;
        fn emit(&self, input: anyhow::Result<DataDto>) -> Self::ViewModel {
            match input {
                Ok(input) => {
                    Ok(PresentationalDataA {
                        id: input.0,
                        name: input.1
                    })
                },
                Err(_) => {
                    Err(anyhow::Error::msg("error"))
                }
            }
        }
    }

    #[tokio::test]
    pub async fn controller() -> anyhow::Result<()> {
        let input = RequestForm {
            name: "Test Man".to_string()
        };

        pub async fn handling(input: DataDto) -> anyhow::Result<DataDto> {
            Ok(input)
        }

        let res = Controller::new(TransformerA, PresenterA)
            .transform(input)
            .handle(|input| async {
                handling(input).await
            }).await?;

        println!("{}", serde_json::to_string(&res)?);
        Ok(())
    }
}
