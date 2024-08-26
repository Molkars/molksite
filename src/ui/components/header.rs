use derive_builder::Builder;
use derive_more::Constructor;
use macros::html;
use crate::html::HtmlBundle;

#[derive(Builder, Default)]
#[builder(pattern = "owned", default, setter(into))]
pub struct Header {
    #[builder(setter(strip_option))]
    pub logo: Option<HeaderLogo>,
    #[builder(setter(each(name = "destination")))]
    pub destinations: Vec<Destination>,
    #[builder(setter(each(name = "button")))]
    pub buttons: Vec<HeaderButton>,
}

#[derive(Constructor)]
pub struct Destination {
    pub href: String,
    pub name: HtmlBundle,
}

#[derive(Constructor)]
pub struct HeaderLogo {
    pub href: String,
    pub label: String,
    pub logo: HtmlBundle,
}

#[derive(Constructor)]
pub struct HeaderButton {
    pub content: HtmlBundle,
}

impl Into<HtmlBundle> for Header {
    fn into(self) -> HtmlBundle {
        html! {
<header class="bg-white">
  <div class="mx-auto max-w-screen-xl px-4 sm:px-6 lg:px-8">
    <div class="flex h-16 items-center justify-between">
      <div class="md:flex md:items-center md:gap-12">
          if let Some(logo) = &self.logo {
            <a class="block text-indigo-600" href=(logo.href)>
              <span class="sr-only">(logo.label)</span>
              (logo.logo)
            </a>
          }
      </div>

      <div class="hidden md:block">
        <nav aria-label="Global">
          <ul class="flex items-center gap-6 text-sm">
            for destination in &self.destinations {
              <li>
                <a
                  class="text-indigo-500 transition hover:text-gray-500/75"
                  href=(destination.href)
                >(destination.name)</a>
              </li>
            }
          </ul>
        </nav>
      </div>

      <div class="flex items-center gap-4">
        <div class="sm:flex sm:gap-4">
          for button in self.buttons {
            (button.content)
          }
        </div>

        // TODO: drawer
        <div class="block md:hidden">
          <button class="rounded bg-gray-100 p-2 text-gray-600 transition hover:text-gray-600/75">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="size-5"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
              stroke-width="2"
            >
              <path stroke-linecap="round" stroke-linejoin="round" d="M4 6h16M4 12h16M4 18h16" />
            </svg>
          </button>
        </div>
      </div>
    </div>
  </div>
</header>
        }
    }
}