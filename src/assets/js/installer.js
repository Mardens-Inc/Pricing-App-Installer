const installLocationInput = $("input#install-location");
const createDesktopShortcutToggle = $("toggle#create-desktop-shortcut");
const createStartMenuShortcutToggle = $("toggle#create-start-menu-shortcut");
const createQuickLaunchShortcutToggle = $("toggle#create-quick-launch-shortcut");
const createTaskbarShortcutToggle = $("toggle#create-taskbar-shortcut");
const startWithWindows = $("toggle#start-with-windows");


let options = {
    location: installLocationInput.val(),
    createDesktopShortcut: createDesktopShortcutToggle.attr("value") === "true",
    createStartMenuShortcut: createStartMenuShortcutToggle.attr("value") === "true",
    createQuickLaunchShortcut: createQuickLaunchShortcutToggle.attr('value') === "true",
    createTaskbarShortcut: createTaskbarShortcutToggle.attr('value') === "true",
    startWithWindows: startWithWindows.attr('value') === "true"
};

createQuickLaunchShortcutToggle.on("toggle", (_, value) => {
    options.createQuickLaunchShortcut = value;
})
createDesktopShortcutToggle.on("toggle", (_, value) => {
    options.createDesktopShortcut = value;
})
createStartMenuShortcutToggle.on("toggle", (_, value) => {
    options.createStartMenuShortcut = value;
})
createTaskbarShortcutToggle.on("toggle", (_, value) => {
    options.createTaskbarShortcut = value;
})

startWithWindows.on("toggle", (_, value) => {
    options.startWithWindows = value;
});

$("#select-install-location").on("click", async () => {
    let location = await window.__TAURI__.dialog.open({
        defaultPath: options.location,
        directory: true
    })
    console.log(location)
    if (location === null || location === "") return;
    location = `${location}\\Mardens Inc.\\Pricing App`;
    installLocationInput.val(location);
    options.location = location;
});

$("#install").on("click", async () => {

});