document.addEventListener("DOMContentLoaded", function () {
  const numberInput = document.getElementById("number-input");
  const wordsOutput = document.getElementById("words-output");
  const ordinalOutput = document.getElementById("ordinal-output");
  const ordinalWordsOutput = document.getElementById("ordinal-words-output");
  const optionsSelect = document.getElementById("options");
  const codeExample = document.getElementById("code-example");

  // Set a default value
  numberInput.value = "12345";

  // Initial conversion
  convertNumber();

  numberInput.addEventListener("input", convertNumber);
  numberInput.addEventListener("keypress", function (e) {
    if (e.key === "Enter") {
      convertNumber();
    }
  });

  function convertNumber() {
    const number = numberInput.value.trim();
    if (!number) {
      showError("Please enter a number");
      return;
    }

    try {
      // Get all selected options
      const selectedOptions = Array.from(optionsSelect.selectedOptions).map(option => option.value);
      
      const options = {
        useCommas: selectedOptions.includes("use-commas"),
        useAnd: selectedOptions.includes("use-and"),
        appendOnly: selectedOptions.includes("append-only"),
        uppercase: selectedOptions.includes("uppercase"),
        capitalize: selectedOptions.includes("capitalize"),
      };

      // Update the code example
      updateCodeExample(number, options);

      // Convert to words
      const words = numlang.toWords(number, options);
      wordsOutput.textContent = words;
      wordsOutput.classList.remove("error");

      try {
        // Only attempt ordinal conversions for integers
        if (Number.isInteger(Number(number))) {
          // Convert to ordinal
          const ordinal = numlang.toOrdinal(number);
          ordinalOutput.textContent = ordinal;
          ordinalOutput.classList.remove("error");

          // Convert to ordinal words
          const ordinalWords = numlang.toWordsOrdinal(number, options);
          ordinalWordsOutput.textContent = ordinalWords;
          ordinalWordsOutput.classList.remove("error");
        } else {
          ordinalOutput.textContent = "Ordinals only work with integers";
          ordinalOutput.classList.add("error");
          ordinalWordsOutput.textContent = "Ordinals only work with integers";
          ordinalWordsOutput.classList.add("error");
        }
      } catch (err) {
        ordinalOutput.textContent = err.message;
        ordinalOutput.classList.add("error");
        ordinalWordsOutput.textContent = err.message;
        ordinalWordsOutput.classList.add("error");
      }
    } catch (err) {
      showError(err.message);
    }
  }

  function showError(message) {
    wordsOutput.textContent = message;
    wordsOutput.classList.add("error");
    ordinalOutput.textContent = "-";
    ordinalWordsOutput.textContent = "-";
  }

  function updateCodeExample(number, options) {
    const optionsStr = Object.entries(options)
      .map(([key, value]) => `  ${key}: ${value}`)
      .join(",\n");

    let codeSnippet = `import numlang from 'numlang';

// Options
const options = {
${optionsStr}
};

// Convert to words
const words = numlang.toWords(${number}, options);
// "${numlang.toWords(number, options)}"
`;

    if (Number.isInteger(Number(number))) {
      codeSnippet += `
// Convert to ordinal
const ordinal = numlang.toOrdinal(${number});
// "${numlang.toOrdinal(number)}"

// Convert to ordinal words
const ordinalWords = numlang.toWordsOrdinal(${number}, options);
// "${numlang.toWordsOrdinal(number, options)}"`;
    } else {
      codeSnippet += `
// Ordinal conversions only work with integers`;
    }

    codeExample.innerHTML = `<pre><code>${codeSnippet}</code></pre>`;
  }

  // Event listener for the multiselect
  optionsSelect.addEventListener("change", convertNumber);
});
