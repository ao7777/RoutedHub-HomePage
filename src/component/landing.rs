use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use yew::{classes, function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct LetterBarProps {
    pub letter: char,
    pub index: usize,
}

#[derive(Debug, EnumIter)]
enum LetterBarFace {
    Back,
    Left,
    Right,
}

impl std::fmt::Display for LetterBarFace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LetterBarFace::Back => write!(f, "back"),
            LetterBarFace::Left => write!(f, "left"),
            LetterBarFace::Right => write!(f, "right"),
        }
    }
}

#[function_component(LetterBar)]
pub fn letter_bar(props: &LetterBarProps) -> Html {
    html! {
        <div class="w-48 spin-bar-container py-24" >
            <div class="spin-bar-cube" style={format!("
  animation: spin {}ms cubic-bezier(0.175,0.885,0.320,1.275);",2000+props.index *300)}>
            {
                for LetterBarFace::iter().map(|i| html!{
                    <div class={classes!(format!("spin-bar-side-{}",i),"spin-bar","h-screen")} />
                })
            }
            <div class="h-screen items-center bg-slate-900/30 ease-in-out delay-700 spin-bar spin-bar-side-front">
                <div class="font-extrabold text-xll text-white font-mono italic bg-clip-text px-2"
                style="-webkit-text-stroke: 1px #a0494d; letter-spacing: -0.4em; line-height:40rem">
                { props.letter }
                </div>
            </div>
        </div>
        </div>
    }
}
