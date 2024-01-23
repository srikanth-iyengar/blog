use leptos::*;
use leptos_meta::provide_meta_context;
use leptos_router::*;

use crate::components::footer::Footer;
use crate::components::navbar::Navbar;

pub mod components;

fn main() {
    provide_meta_context();
    mount_to_body(|| {
        view! {
            <div id="root">
                <Router>
                    <Navbar />
                    <main>
                        <Routes>
                            <Route path="/" view=move || view! {<p>Home</p>} />
                        </Routes>
                        <Footer />
                    </main>
                </Router>
            </div>
        }
    });
}
