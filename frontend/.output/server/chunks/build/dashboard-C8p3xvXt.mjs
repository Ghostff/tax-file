import { _ as __nuxt_component_0 } from './nuxt-link-CrQNpPg8.mjs';
import { ref, mergeProps, unref, withCtx, createTextVNode, useSSRContext } from 'vue';
import { ssrRenderAttrs, ssrRenderAttr, ssrInterpolate, ssrRenderList, ssrRenderClass, ssrRenderComponent } from 'vue/server-renderer';
import { u as useAuth } from './useAuth-BgPgYEZC.mjs';
import { u as useRouter } from './server.mjs';
import '../nitro/nitro.mjs';
import 'node:http';
import 'node:https';
import 'node:events';
import 'node:buffer';
import 'node:fs';
import 'node:path';
import 'node:crypto';
import 'node:url';
import '../routes/renderer.mjs';
import 'vue-bundle-renderer/runtime';
import 'unhead/server';
import 'devalue';
import 'unhead/utils';
import 'unhead/plugins';
import 'pinia';
import 'vue-router';

const _sfc_main = {
  __name: "dashboard",
  __ssrInlineRender: true,
  setup(__props) {
    var _a, _b, _c;
    const { user, isLoggedIn } = useAuth();
    const profileForm = ref({
      first_name: ((_a = user.value) == null ? void 0 : _a.first_name) || "",
      last_name: ((_b = user.value) == null ? void 0 : _b.last_name) || "",
      email: ((_c = user.value) == null ? void 0 : _c.email) || "",
      password: ""
    });
    const showUpload = ref(false);
    const uploading = ref(false);
    const documents = ref([]);
    const manualItems = ref([{ label: "", amount: 0 }]);
    const taxData = ref([]);
    const showAI = ref(false);
    const aiQuestion = ref("");
    const chatHistory = ref([
      { role: "assistant", content: "Hello! I am your tax helper. I have access to your tax records. How can I help you today?" }
    ]);
    ref(null);
    useRouter();
    return (_ctx, _push, _parent, _attrs) => {
      const _component_NuxtLink = __nuxt_component_0;
      _push(`<div${ssrRenderAttrs(mergeProps({ class: "max-w-4xl mx-auto py-8 px-4 sm:px-6 lg:px-8" }, _attrs))}>`);
      if (unref(isLoggedIn)) {
        _push(`<div><div class="md:grid md:grid-cols-3 md:gap-6"><div class="md:col-span-1"><div class="px-4 sm:px-0"><h3 class="text-lg font-medium leading-6 text-gray-900">Profile</h3><p class="mt-1 text-sm text-gray-600">Update your account information.</p></div></div><div class="mt-5 md:mt-0 md:col-span-2"><form><div class="shadow sm:rounded-md sm:overflow-hidden"><div class="px-4 py-5 bg-white space-y-6 sm:p-6"><div class="grid grid-cols-6 gap-6"><div class="col-span-6 sm:col-span-3"><label class="block text-sm font-medium text-gray-700">First name</label><input${ssrRenderAttr("value", unref(profileForm).first_name)} type="text" class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm py-2 px-3 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"></div><div class="col-span-6 sm:col-span-3"><label class="block text-sm font-medium text-gray-700">Last name</label><input${ssrRenderAttr("value", unref(profileForm).last_name)} type="text" class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm py-2 px-3 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"></div><div class="col-span-6"><label class="block text-sm font-medium text-gray-700">Email address</label><input${ssrRenderAttr("value", unref(profileForm).email)} type="email" class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm py-2 px-3 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"></div><div class="col-span-6"><label class="block text-sm font-medium text-gray-700">New Password (optional)</label><input${ssrRenderAttr("value", unref(profileForm).password)} type="password" class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm py-2 px-3 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"></div></div></div><div class="px-4 py-3 bg-gray-50 text-right sm:px-6 flex justify-between"><button type="button" class="inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-red-600 hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500"> Delete Account </button><button type="submit" class="inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"> Save </button></div></div></form></div></div><div class="hidden sm:block" aria-hidden="true"><div class="py-5"><div class="border-t border-gray-200"></div></div></div><div class="md:grid md:grid-cols-3 md:gap-6 mt-10"><div class="md:col-span-1"><div class="px-4 sm:px-0"><h3 class="text-lg font-medium leading-6 text-gray-900">Tax Filing</h3><p class="mt-1 text-sm text-gray-600">File your taxes for the current year or view history.</p></div></div><div class="mt-5 md:mt-0 md:col-span-2"><div class="bg-white shadow sm:rounded-lg p-6"><div class="flex items-center justify-between mb-6"><h4 class="text-lg font-medium">File Tax for 2024</h4><button class="bg-green-600 text-white px-4 py-2 rounded-md hover:bg-green-700">${ssrInterpolate(unref(showUpload) ? "Cancel" : "Start Filing")}</button></div>`);
        if (unref(showUpload)) {
          _push(`<div class="space-y-6"><div class="border-2 border-dashed border-gray-300 rounded-lg p-12 text-center"><input type="file" class="hidden">`);
          if (!unref(uploading)) {
            _push(`<div class="space-y-1"><p class="text-gray-600">Upload W2 or 1099</p><button class="text-indigo-600 font-medium hover:text-indigo-500"> Click to select a file </button></div>`);
          } else {
            _push(`<div class="text-indigo-600">Uploading and extracting data...</div>`);
          }
          _push(`</div><div class="relative"><div class="absolute inset-0 flex items-center" aria-hidden="true"><div class="w-full border-t border-gray-300"></div></div><div class="relative flex justify-center text-sm"><span class="px-2 bg-white text-gray-500">Or enter manually</span></div></div><div class="space-y-4"><!--[-->`);
          ssrRenderList(unref(manualItems), (item, index) => {
            _push(`<div class="p-4 border rounded-md relative"><button class="absolute top-2 right-2 text-red-500">\xD7</button><div class="grid grid-cols-2 gap-4"><input${ssrRenderAttr("value", item.label)} placeholder="Description (e.g. Interest)" class="border rounded px-2 py-1"><input${ssrRenderAttr("value", item.amount)} type="number" placeholder="Amount" class="border rounded px-2 py-1"></div></div>`);
          });
          _push(`<!--]--><button class="text-sm text-indigo-600 font-medium">+ Add item</button><div class="text-right"><button class="bg-indigo-600 text-white px-4 py-2 rounded-md hover:bg-indigo-700">Save Manual Info</button></div></div></div>`);
        } else {
          _push(`<!---->`);
        }
        if (unref(taxData).length > 0) {
          _push(`<div class="mt-8"><h5 class="font-medium mb-3">Extracted Tax Summary</h5><div class="space-y-4"><!--[-->`);
          ssrRenderList(unref(taxData), (data) => {
            _push(`<div class="border rounded-md p-4 bg-gray-50"><h6 class="font-bold text-indigo-700">Year: ${ssrInterpolate(data.year)}</h6><div class="mt-2 text-sm">`);
            if (data.data.documents && data.data.documents.length > 0) {
              _push(`<div><!--[-->`);
              ssrRenderList(data.data.documents, (doc, idx) => {
                _push(`<div class="mt-1 pb-1 border-b last:border-0"><span class="font-medium">${ssrInterpolate(doc.type)}:</span>`);
                if (doc.extracted.employer) {
                  _push(`<span class="ml-1">Employer: ${ssrInterpolate(doc.extracted.employer)}, Wages: $${ssrInterpolate(doc.extracted.wages)}</span>`);
                } else if (doc.extracted.payer) {
                  _push(`<span class="ml-1">Payer: ${ssrInterpolate(doc.extracted.payer)}, Income: $${ssrInterpolate(doc.extracted.interest_income)}</span>`);
                } else {
                  _push(`<!---->`);
                }
                _push(`</div>`);
              });
              _push(`<!--]--></div>`);
            } else {
              _push(`<!---->`);
            }
            if (data.data.manual && data.data.manual.length > 0) {
              _push(`<div class="mt-2 pt-2 border-t"><p class="italic text-gray-500">Manual Entries:</p><!--[-->`);
              ssrRenderList(data.data.manual, (m, idx) => {
                _push(`<div class="ml-2"> - ${ssrInterpolate(m.label)}: $${ssrInterpolate(m.amount)}</div>`);
              });
              _push(`<!--]--></div>`);
            } else {
              _push(`<!---->`);
            }
            _push(`</div></div>`);
          });
          _push(`<!--]--></div></div>`);
        } else {
          _push(`<!---->`);
        }
        if (unref(documents).length > 0) {
          _push(`<div class="mt-8"><h5 class="font-medium mb-3">Previous Uploads</h5><ul class="divide-y divide-gray-200"><!--[-->`);
          ssrRenderList(unref(documents), (doc) => {
            _push(`<li class="py-3 flex justify-between items-center"><div><span class="font-medium">${ssrInterpolate(doc.year)} ${ssrInterpolate(doc.document_type)}</span><span class="text-sm text-gray-500 ml-2">${ssrInterpolate(doc.file_name)}</span></div><button class="text-indigo-600 hover:text-indigo-900 text-sm font-medium">Download</button></li>`);
          });
          _push(`<!--]--></ul></div>`);
        } else {
          _push(`<!---->`);
        }
        _push(`</div></div></div><div class="fixed bottom-4 right-4 w-80 md:w-96">`);
        if (unref(showAI)) {
          _push(`<div class="bg-white rounded-lg shadow-xl border flex flex-col h-96"><div class="p-4 bg-indigo-600 text-white rounded-t-lg flex justify-between items-center"><span class="font-medium">Tax AI Helper</span><button class="text-white hover:text-gray-200">\xD7</button></div><div class="flex-1 p-4 overflow-y-auto space-y-4"><!--[-->`);
          ssrRenderList(unref(chatHistory), (msg, i) => {
            _push(`<div class="${ssrRenderClass(msg.role === "user" ? "text-right" : "text-left")}"><span class="${ssrRenderClass([msg.role === "user" ? "bg-indigo-100" : "bg-gray-100", "inline-block px-3 py-2 rounded-lg text-sm max-w-[80%]"])}">${ssrInterpolate(msg.content)}</span></div>`);
          });
          _push(`<!--]--></div><div class="p-3 border-t"><form class="flex gap-2"><input${ssrRenderAttr("value", unref(aiQuestion))} placeholder="Ask a tax question..." class="flex-1 border rounded-md px-3 py-1 text-sm"><button type="submit" class="bg-indigo-600 text-white px-3 py-1 rounded-md text-sm">Ask</button></form></div></div>`);
        } else {
          _push(`<button class="bg-indigo-600 text-white p-4 rounded-full shadow-lg hover:bg-indigo-700"><span class="sr-only">AI Helper</span> \u{1F916} </button>`);
        }
        _push(`</div></div>`);
      } else {
        _push(`<div class="text-center py-20"><h2 class="text-2xl font-bold">Welcome to Tax File App</h2><p class="mt-4">Please `);
        _push(ssrRenderComponent(_component_NuxtLink, {
          to: "/login",
          class: "text-indigo-600"
        }, {
          default: withCtx((_, _push2, _parent2, _scopeId) => {
            if (_push2) {
              _push2(`Login`);
            } else {
              return [
                createTextVNode("Login")
              ];
            }
          }),
          _: 1
        }, _parent));
        _push(` or `);
        _push(ssrRenderComponent(_component_NuxtLink, {
          to: "/register",
          class: "text-indigo-600"
        }, {
          default: withCtx((_, _push2, _parent2, _scopeId) => {
            if (_push2) {
              _push2(`Register`);
            } else {
              return [
                createTextVNode("Register")
              ];
            }
          }),
          _: 1
        }, _parent));
        _push(` to continue.</p></div>`);
      }
      _push(`</div>`);
    };
  }
};
const _sfc_setup = _sfc_main.setup;
_sfc_main.setup = (props, ctx) => {
  const ssrContext = useSSRContext();
  (ssrContext.modules || (ssrContext.modules = /* @__PURE__ */ new Set())).add("pages/dashboard.vue");
  return _sfc_setup ? _sfc_setup(props, ctx) : void 0;
};

export { _sfc_main as default };
//# sourceMappingURL=dashboard-C8p3xvXt.mjs.map
