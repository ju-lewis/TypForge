
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


    // Idk, nice visual loading animation lol
    const id = setInterval(() => {
        if (elem.disabled) {
            elem.placeholder = "Loading" + elem.dots;
            elem.dots += ".";
            if (elem.dots.length > 4) {
                elem.dots = "";
                clearInterval(id);
                elem.placeholder = "Content Loaded";
            }
        } 
    }, 100);

}

function resetInput(id) {

    const elem = document.getElementById(id);

    elem.disabled = false;
    
    if (id === "cv-content") {
        elem.placeholder = "CV content";
        cvContent = "";
    } else if (id === "app-content") {
        elem.placeholder = "Application details";
        appContent = "";
    }

}

async function sendInputs() {

    const body = {
        cv: cvContent,
        spec: appContent
    };

    const result = await fetch("/template", {
        method: "POST",
        headers: {"Content-Type": "application/json"},
        body: JSON.stringify(body)
    });

    if (!result.ok) {
        //TODO: Warn user there was an error (check status code)
        return;
    }
    let typstCode = await result.text();

    // Sanitise code
    typstCode = typstCode.replaceAll("```typst", "");
    typstCode = typstCode.replaceAll("```", "");
    
    
    const outputBox = document.getElementById("typst-input");
    outputBox.innerHTML = typstCode;
}



