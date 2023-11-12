import { debounce, Notice, PluginSettingTab, Setting } from "obsidian";

import type { App } from "obsidian";
import type MainPlugin from "../main";

export class MainPluginSettingTab extends PluginSettingTab {
    plugin: MainPlugin;

    constructor(app: App, plugin: MainPlugin) {
        super(app, plugin);
        this.plugin = plugin;
    }

    display(): void {
        const { containerEl } = this;
        containerEl.empty();

        // Settings Header
        containerEl.createEl("h1", { text: "Formatto" });
        containerEl
            .createEl("span", { text: "Obsidian Formatter by " })
            .createEl("a", {
                text: "Deca",
                href: "https://github.com/decaplanet",
            });
        containerEl.createEl("span", { text: ".\n" });
        containerEl.createEl("p", {
            text: "All values should be at least 0.",
            cls: "formatto__paragraph-margin formatto__important",
        });

        containerEl.createEl("div", { cls: "formatto__vertical-gap" });

        const debounceMsg = debounce(
            (text: string, value: string) => {
                if (
                    value !== "" &&
                    (isNaN(parseInt(value)) || parseInt(value) < 0)
                ) {
                    new Notice(text);
                }
            },
            1000,
            true
        );

        // Heading Gaps
        containerEl.createEl("h2", {
            text: "Heading Gaps",
        });
        new Setting(containerEl)
            .setName("Before Top Level Headings")
            .setDesc("Decides gaps before highest level of headings.")
            .addText((text) =>
                text
                    .setPlaceholder("3")
                    .setValue(
                        this.plugin.settings.headingGaps.beforeTopLevelHeadings
                    )
                    .onChange(async (value) => {
                        if (
                            value !== "" &&
                            (isNaN(parseInt(value)) || parseInt(value) < 0)
                        ) {
                            debounceMsg(
                                "Please enter a valid number.\nIt should be at least 0.",
                                value
                            );
                        }

                        this.plugin.settings.headingGaps.beforeTopLevelHeadings =
                            value;
                        await this.plugin.saveSettings();
                    })
            );
        new Setting(containerEl)
            .setName("Before First Sub Heading")
            .setDesc(
                "Decides the child heading gap right before a parent heading."
            )
            .addText((text) =>
                text
                    .setPlaceholder("1")
                    .setValue(
                        this.plugin.settings.headingGaps.beforeFirstSubHeading
                    )
                    .onChange(async (value) => {
                        if (
                            value !== "" &&
                            (isNaN(parseInt(value)) || parseInt(value) < 0)
                        ) {
                            debounceMsg(
                                "Please enter a valid number.\nIt should be at least 0.",
                                value
                            );
                        }

                        this.plugin.settings.headingGaps.beforeFirstSubHeading =
                            value;
                        await this.plugin.saveSettings();
                    })
            );
        new Setting(containerEl)
            .setName("Before Sub Headings")
            .setDesc(
                "Decides gaps before headings that are not in the highest level."
            )
            .addText((text) =>
                text
                    .setPlaceholder("2")
                    .setValue(
                        this.plugin.settings.headingGaps.beforeSubHeadings
                    )
                    .onChange(async (value) => {
                        if (
                            value !== "" &&
                            (isNaN(parseInt(value)) || parseInt(value) < 0)
                        ) {
                            debounceMsg(
                                "Please enter a valid number.\nIt should be at least 0.",
                                value
                            );
                        }

                        this.plugin.settings.headingGaps.beforeSubHeadings =
                            value;
                        await this.plugin.saveSettings();
                    })
            );

        // Other Gaps
        containerEl.createEl("h2", {
            text: "Other Gaps",
        });
        new Setting(containerEl)
            .setName("After Properties")
            .setDesc("Decides the gap after a YAML properties.")
            .addText((text) =>
                text
                    .setPlaceholder("2")
                    .setValue(this.plugin.settings.otherGaps.afterProperties)
                    .onChange(async (value) => {
                        if (
                            value !== "" &&
                            (isNaN(parseInt(value)) || parseInt(value) < 0)
                        ) {
                            debounceMsg(
                                "Please enter a valid number.\nIt should be at least 0.",
                                value
                            );
                        }

                        this.plugin.settings.otherGaps.afterProperties = value;
                        await this.plugin.saveSettings();
                    })
            );
        new Setting(containerEl)
            .setName("Before Contents")
            .setDesc(
                "Decides gaps before contents (ex: Text section right before headings)."
            )
            .addText((text) =>
                text
                    .setPlaceholder("0")
                    .setValue(this.plugin.settings.otherGaps.beforeContents)
                    .onChange(async (value) => {
                        if (
                            value !== "" &&
                            (isNaN(parseInt(value)) || parseInt(value) < 0)
                        ) {
                            debounceMsg(
                                "Please enter a valid number.\nIt should be at least 0.",
                                value
                            );
                        }

                        this.plugin.settings.otherGaps.beforeContents = value;
                        await this.plugin.saveSettings();
                    })
            );
        new Setting(containerEl)
            .setName("Before Contents After Code Blocks")
            .setDesc(
                "Decides gaps before contents that are right after code blocks."
            )
            .addText((text) =>
                text
                    .setPlaceholder("0")
                    .setValue(
                        this.plugin.settings.otherGaps
                            .beforeContentsAfterCodeBlocks
                    )
                    .onChange(async (value) => {
                        if (
                            value !== "" &&
                            (isNaN(parseInt(value)) || parseInt(value) < 0)
                        ) {
                            debounceMsg(
                                "Please enter a valid number.\nIt should be at least 0.",
                                value
                            );
                        }

                        this.plugin.settings.otherGaps.beforeContentsAfterCodeBlocks =
                            value;
                        await this.plugin.saveSettings();
                    })
            );
        new Setting(containerEl)
            .setName("Before Code Blocks")
            .setDesc("Decides gaps before code blocks.")
            .addText((text) =>
                text
                    .setPlaceholder("0")
                    .setValue(this.plugin.settings.otherGaps.beforeCodeBlocks)
                    .onChange(async (value) => {
                        if (
                            value !== "" &&
                            (isNaN(parseInt(value)) || parseInt(value) < 0)
                        ) {
                            debounceMsg(
                                "Please enter a valid number.\nIt should be at least 0.",
                                value
                            );
                        }

                        this.plugin.settings.otherGaps.beforeCodeBlocks = value;
                        await this.plugin.saveSettings();
                    })
            );
        new Setting(containerEl)
            .setName("Before Code Blocks After Headings")
            .setDesc(
                "Decides gaps before code blocks that are right after headings."
            )
            .addText((text) =>
                text
                    .setPlaceholder("0")
                    .setValue(
                        this.plugin.settings.otherGaps
                            .beforeCodeBlocksAfterHeadings
                    )
                    .onChange(async (value) => {
                        if (
                            value !== "" &&
                            (isNaN(parseInt(value)) || parseInt(value) < 0)
                        ) {
                            debounceMsg(
                                "Please enter a valid number.\nIt should be at least 0.",
                                value
                            );
                        }

                        this.plugin.settings.otherGaps.beforeCodeBlocksAfterHeadings =
                            value;
                        await this.plugin.saveSettings();
                    })
            );
    }
}
