
const typstInput = document.getElementById("typst-input");

let cvContent = "";
let appContent = "";


/**
 *  Changes the size of the `typstInput` textarea to match the number of newlines.
 *  This allows the box to grow dynamically.
 */
function autoSizeInput() {
    const numNewLines = Math.max([...typstInput.value.matchAll(/\n/g)].length + 2, 10);

    //TODO: Implement limit

    typstInput.rows = numNewLines;

}

function handleInput(elem) {

    // Get input content
    if (elem.id === "cv-content") {
        cvContent = elem.value;
    } else if (elem.id === "app-content") {
        appContent = elem.value;
    } else return;

    // Replace displayed content in textbox with loading symbol
    setTimeout(() => {
        elem.value = "";
        textBoxLoading(elem);
    }, 150);

}

function textBoxLoading(elem) {
    elem.placeholder = "Loading";
    elem.disabled = true;
    elem.dots = "";

    const id = setInterval(() => {
        if (elem.disabled) {
            elem.placeholder = "Loading" + elem.dots;
            elem.dots += ".";
            if (elem.dots.length > 3) {
                elem.dots = "";
            }
        } else {
            clearInterval(id);
        }
    }, 300);
}


