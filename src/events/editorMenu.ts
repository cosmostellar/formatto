import { getLocale, LOCALE_CATEGORY } from "src/lang/getLocale";

import FormattoPlugin from "../main";

export class FormattoEditorMenu {
    private plugin: FormattoPlugin;

    constructor(plugin: FormattoPlugin) {
        this.plugin = plugin;
    }

    registerEditorMenus() {
        this.getEventsArr().forEach((item) => {
            this.plugin.registerEvent(item);
        });
    }

    private getEventsArr() {
        return [
            this.plugin.app.workspace.on("editor-menu", (menu, editor) => {
                menu.addItem((item) =>
                    item
                        .setTitle(
                            getLocale(
                                LOCALE_CATEGORY.EDITOR_MENU,
                                "Format Document"
                            )
                        )
                        .setIcon("formatto-logo")
                        .onClick(() => {
                            this.plugin.utils.formatDocument(editor);
                        })
                );
            }),
        ];
    }
}
