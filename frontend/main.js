
const typstInput = document.getElementById("typst-input");


/**
 *  Changes the size of the `typstInput` textarea to match the number of newlines.
 *  This allows the box to grow dynamically.
 */
function autoSizeInput() {
    const numNewLines = [...typstInput.value.matchAll(/\n/g)].length + 2;

    //TODO: Implement limit

    typstInput.rows = numNewLines;

}


