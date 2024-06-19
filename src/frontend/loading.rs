use crate::backend::LoadingStep;
use gtk::prelude::*;
use relm4::prelude::*;

#[derive(Debug)]
pub enum LoadingInput {
    BackendLoading {
        step: LoadingStep,
        progress: f32,
        message: String,
    },
}

#[derive(Debug)]
pub struct LoadingView {
    title: String,
    message: String,
    progress: f64,
}

#[relm4::component(pub)]
impl Component for LoadingView {
    type Init = ();
    type Input = LoadingInput;
    type Output = ();
    type CommandOutput = ();

    view! {
        #[root]
        gtk::Box {
            set_halign: gtk::Align::Center,
            set_valign: gtk::Align::Center,
            set_vexpand: true,
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 10,

            gtk::Label {
                #[watch]
               set_markup: &format!("<span size=\"medium\" weight=\"medium\">{}</span>", &model.title),
            },

            gtk::ProgressBar {
                #[watch]
                set_fraction: model.progress,
            },

            gtk::Label {
                #[watch]
                set_markup: &format!("<span color=\"grey\">{}</span>", &model.message),
            },
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            title: String::new(),
            message: String::new(),
            progress: 0.0,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, input: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        self.process_input(input);
    }
}

impl LoadingView {
    fn process_input(&mut self, input: LoadingInput) {
        match input {
            LoadingInput::BackendLoading {
                step,
                progress,
                message,
            } => {
                self.progress = (progress / 100.0) as f64;
                self.message = message;
                self.title = match step {
                    LoadingStep::LoadingConfiguration => "Loading configuration".to_string(),
                    LoadingStep::ReadingConfiguration => "Reading configuration".to_string(),
                    LoadingStep::ConfigurationReadSuccessfully => "Read configuration".to_string(),
                    LoadingStep::CheckingConfiguration => "Checking configuration".to_string(),
                    LoadingStep::ConfigurationIsValid => "Checking configuration".to_string(),
                    LoadingStep::DecodingChainSpecification => {
                        "Decoding chain specification".to_string()
                    }
                    LoadingStep::DecodedChainSpecificationSuccessfully => {
                        "Decoding chain specification".to_string()
                    }
                    LoadingStep::CheckingNodePath => "Checking node path".to_string(),
                    LoadingStep::CreatingNodePath => "Creating node path".to_string(),
                    LoadingStep::NodePathReady => "Node path ready".to_string(),
                    LoadingStep::PreparingNetworkingStack => {
                        "Preparing networking stack".to_string()
                    }
                    LoadingStep::ReadingNetworkKeypair => "Reading network keypair".to_string(),
                    LoadingStep::GeneratingNetworkKeypair => {
                        "Generating network keypair".to_string()
                    }
                    LoadingStep::WritingNetworkKeypair => {
                        "Writing network keypair to disk".to_string()
                    }
                    LoadingStep::InstantiatingNetworkingStack => {
                        "Instantiating networking stack".to_string()
                    }
                    LoadingStep::NetworkingStackCreatedSuccessfully => {
                        "Networking stack created successfully".to_string()
                    }
                    LoadingStep::CreatingConsensusNode => "Creating consensus node".to_string(),
                    LoadingStep::ConsensusNodeCreatedSuccessfully => {
                        "Consensus node created successfully".to_string()
                    }
                    LoadingStep::CreatingFarmer => "Creating farmer".to_string(),
                    LoadingStep::FarmerCreatedSuccessfully => {
                        "Farmer created successfully".to_string()
                    }
                    LoadingStep::WipingFarm => "Wiping farm".to_string(),
                    LoadingStep::WipedFarmSuccessfully => "Wiped farm successfully".to_string(),
                    LoadingStep::WipingNode => "Wiping node".to_string(),
                    LoadingStep::WipedNodeSuccessfully => "Wiped node successfully".to_string(),
                };
            }
        }
    }
}
