use yew::prelude::*;
use reqwasm::http::Request;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Properties, PartialEq, Clone, Deserialize, Debug)]
struct Student {
    name: String,
    age: u8,
    grade: u8,
}

impl Display for Student {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("{} is {} years old and got {}/20 on their exam.", self.name, self.age, self.grade).fmt(f)
    }
}

#[function_component(App)]
fn app() -> Html {
    let students = use_state(|| vec![] );
    {
        let students = students.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_students: Vec<Student> = Request::get("students")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                students.set(fetched_students);
            });
            || ()
        }, ());
    }
    
    let students_string = students
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    html! {
        <>
            <h1>{ "Students" }</h1>
            <p>{ format!("{}", students_string)}</p>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}