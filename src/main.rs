#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};
use rocket::{form::Form, serde::Serialize};

#[cfg(test)] mod tests;

#[derive(Serialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
struct Submission {
    firstname: String,
    lastname: String,
}

static mut FORM_SUBMISSIONS: Vec<Submission> = vec![];

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/form")]
fn form() -> Template {
   Template::render("form", context! {})
}

#[derive(FromForm)]
struct UserInput {
    firstname: String,
    lastname: String,
}

#[post("/submit", data = "<user_input>")]
fn submit(user_input: Form<UserInput>) -> String {
    unsafe {
        FORM_SUBMISSIONS.push(Submission { firstname: user_input.firstname.clone(), lastname: user_input.lastname.clone() });
    }
    format!("Your value: {}, {}", user_input.firstname, user_input.lastname)
}

#[get("/submissions")]
fn submissions() -> Template {
    unsafe {
        Template::render("submissions", context! { submissions: FORM_SUBMISSIONS.clone() })
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, form, submissions, submit]).attach(Template::fairing())
}
