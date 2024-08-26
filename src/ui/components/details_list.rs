use derive_builder::Builder;
use macros::html;
use crate::html::HtmlBundle;

#[derive(Builder, Default)]
#[builder(pattern="owned", default, setter(into))]
pub struct DetailsList {
    #[builder(setter(each(name="item")))]
    items: Vec<(HtmlBundle, HtmlBundle)>,
}

impl Into<HtmlBundle> for DetailsList {
    fn into(self) -> HtmlBundle {
        html! {
        <div class="flow-root rounded-lg border border-gray-100 py-3 shadow-sm">
            <dl class="-my-3 divide-y divide-gray-100 text-sm">
                for (title, content) in self.items {
                    <div class="grid grid-cols-1 gap-1 p-3 even:bg-gray-50 sm:grid-cols-3 sm:gap-4">
                        <dt class="font-medium text-gray-900">(title)</dt>
                        <dd class="text-gray-700 sm:col-span-2">(content)</dd>
                    </div>
                }
            </dl>
        </div>
        }
    }
}
