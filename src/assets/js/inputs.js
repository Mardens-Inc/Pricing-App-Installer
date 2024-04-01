let debounce = 0;
$(document).on("load", () => {
    clearTimeout(debounce);
    $(`[toggle-hidden]`).each((_, element) => {
        let target = $(element);
        let toggle = $(`#${target.attr("toggle-hidden")}`);
        if (toggle.attr("value") === "true") {
            target.show();
        } else {
            target.hide();
        }
    });
    debounce = setTimeout(() => {
        const toggles = $("toggle");

        // Add a click event listener to all elements with the class "toggle".
        toggles.on("click", (e) => {
            // Prevent the default click behavior.
            e.preventDefault();
            // Get the target of the click event.
            let target = $(e.target);
            // Get the current value of the "value" attribute of the target.
            let value = target.attr("value") === "true";
            // Set the "value" attribute of the target to the opposite of its current value.
            target.attr("value", `${!value}`);
            // Trigger a "toggle" event on the target with the new value.
            target.trigger("toggle", [{value: !value}]);

        });

        // toggles.off("toggle"); // Remove any existing "toggle" event listeners.
        toggles.on("toggle", (e, data) => {
            let target = $(e.target);
            // Toggles the visibility of the target element and the element with the id specified in the "toggle-hidden" attribute of the target.
            $(`[toggle-hidden="${target.attr("id")}"]`).toggle();
            // set the value of the target element to the value of the data object.
            target.attr('value', data.value);
            target.data("events", "toggle")
        });
    }, 500);

})