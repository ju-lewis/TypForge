
const typstInput = document.getElementById("typst-input");
const fileBox = document.getElementById("file-container");
const fileContainer = document.getElementById("file-outer-container");
const alert = document.getElementById("alert");

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
    fileBox.height = typstInput.getBoundingClientRect().height;
}

function handleInput(elem) {

    // Get input content
    if (elem.id === "cv-content") {
        cvContent = elem.value;
    } else if (elem.id === "app-content") {
        appContent = elem.value;
    } else return;

    // Replace displayed content in textbox with loading symbol
    elem.value = "";
    textBoxLoading(elem);

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
    }, 20);

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

        const err = await result.text();
        showAlert(err);
        
        return;
    }
    let typstCode = await result.text();

    // Sanitise code
    typstCode = typstCode.replaceAll("```typst", "");
    typstCode = typstCode.replaceAll("```", "");
    

    typstInput.innerHTML = typstCode;
    autoSizeInput();
    

    requestCompilation();
}

async function requestCompilation() {

    const typst_source = typstInput.value;

    const res = await fetch("/render-pdf", {
        method: "POST",
        headers: {"Content-Type": "application/json"},
        body: JSON.stringify({code: typst_source})
    });

    if (!res.ok) {
        // Compilation failed, warn user

        const err = await res.text();
        showAlert(err);
        return;
    }

    let filename = await res.text();

    updateDocumentSize()

    // Update object to point to new filename
    console.log(`Cached file: ${filename}.pdf`);
    
    const file_box = document.getElementById("file-container");
    file_box.data = `pdf/${filename}.pdf`;
    
}   


// Resize the document to match page resizes
function updateDocumentSize() {

    const rect = fileContainer.getBoundingClientRect();

    fileBox.width = rect.width;

    console.log(`Height: ${typstInput.getBoundingClientRect().height}`);
    fileBox.height = typstInput.getBoundingClientRect().height;
}


function showAlert(msg) {
    alert.querySelector("p").innerText = msg;
    alert.style.display = "block";
    setTimeout(() => {
        alert.style.display = "none";
    }, 5000);
}



window.addEventListener("resize", () => {
    updateDocumentSize();
});
updateDocumentSize();
