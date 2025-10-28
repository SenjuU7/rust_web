use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
struct Model {
    name: String,
    age: u8,
    gender: String,
    email: String,
    password: String,
}

impl Model {
    fn new() -> Self {
        Self {
            name: "".to_string(),
            age: 0,
            gender: "".to_string(),
            email: "".to_string(),
            password: "".to_string(),
        }
    }
}

#[function_component(App)]
fn app() -> Html {
    // State untuk menyimpan data form
    let user = use_state(Model::new);

    // Handlers untuk setiap input field
    let on_name_change = {
        let user = user.clone();
        Callback::from(move |e: InputEvent| {
            let mut new_user = (*user).clone();
            new_user.name = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
            user.set(new_user);
        })
    };

    let on_age_change = {
        let user = user.clone();
        Callback::from(move |e: InputEvent| {
            let mut new_user = (*user).clone();
            let val = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
            new_user.age = val.parse().unwrap_or(0);
            user.set(new_user);
        })
    };

    let on_gender_change = {
        let user = user.clone();
        Callback::from(move |e: InputEvent| {
            let mut new_user = (*user).clone();
            new_user.gender = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
            user.set(new_user);
        })
    };

    let on_email_change = {
        let user = user.clone();
        Callback::from(move |e: InputEvent| {
            let mut new_user = (*user).clone();
            new_user.email = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
            user.set(new_user);
        })
    };

    let on_password_change = {
        let user = user.clone();
        Callback::from(move |e: InputEvent| {
            let mut new_user = (*user).clone();
            new_user.password = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
            user.set(new_user);
        })
    };

    // Handler untuk tombol submit
    let on_submit = {
        let user = user.clone();
        Callback::from(move |_| {
            web_sys::console::log_1(&format!("{:?}", *user).into());
            gloo::dialogs::alert(&format!(
                "Data submitted!\nName: {}\nAge: {}\nGender: {}\nEmail: {}",
                user.name, user.age, user.gender, user.email
            ));
        })
    };

    html! {
        <div style="font-family: sans-serif; max-width: 400px; margin: auto;">
            <h1>{ "User Form" }</h1>

            <label>{ "Name: " }</label>
            <input type="text" value={user.name.clone()} oninput={on_name_change} /><br/><br/>

            <label>{ "Age: " }</label>
            <input type="number" value={user.age.to_string()} oninput={on_age_change} /><br/><br/>

            <label>{ "Gender: " }</label>
            <input type="text" value={user.gender.clone()} oninput={on_gender_change} /><br/><br/>

            <label>{ "Email: " }</label>
            <input type="email" value={user.email.clone()} oninput={on_email_change} /><br/><br/>

            <label>{ "Password: " }</label>
            <input type="password" value={user.password.clone()} oninput={on_password_change} /><br/><br/>

            <button onclick={on_submit}>{ "Submit" }</button>

            <hr/>
            <h3>{ "Current State:" }</h3>
            <pre>{ format!("{:#?}", *user) }</pre>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
