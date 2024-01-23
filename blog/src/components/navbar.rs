use leptos::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <header class="navbar">
            <ul>
                <li class="nav-item"> Home </li>
                <li class="nav-item"> Experience </li>
                <li class="nav-item"> Blogs </li>
                <li class="nav-item"> Projects </li>
            </ul>
        </header>
    }
}
