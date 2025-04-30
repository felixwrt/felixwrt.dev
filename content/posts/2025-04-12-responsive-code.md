+++
title = "Responsive code formatting"
description = "TODO"
+++

The window currently fits <code id="code-width">X</code> characters.

<p>
    <label>
        <input type="checkbox" id="manual-width">
        Manual width:
    </label> 
    <input type="range" id="width-slider" min="30" max="100" value="50" disabled="true" style="vertical-align: middle">
    <code id="code-width-manual">50</code>
</p>
<pre>
    <code id="code-block" style="position: relative"><span id="code-block-inner"></span>
        <div id="vLine" style="position: absolute; top: 0; bottom: 0; width: 3px; background-color: #11b; pointer-events: none; left: 100ch;"></div>
    </code>
</pre>

<script>
    const codeBlock = document.getElementById('code-block');
    const codeBlockInner = document.getElementById('code-block-inner');
    const codeWidth = document.getElementById('code-width');
    const codeWidthManual = document.getElementById('code-width-manual');
    const slider = document.getElementById('width-slider');
    const checkbox = document.getElementById('manual-width');
    const vLine = document.getElementById('vLine');

    async function main() {
        const contents = await fetch("/code-sample.json")
            .then(response => response.json());

        function updateContents(width) {
            let out = "";
            contents.forEach(function (item, idx) {
                const txt_width = item[0];
                const s = item[1];
                if (txt_width < width) {
                    out = s;
                }
            });
            codeBlockInner.textContent = out;
        }

        function update() {
            let charsPerLine = slider.value;
            let charsPerLineWindow = calculateCharsPerLine();
            codeWidth.textContent = charsPerLineWindow;
            if (!checkbox.checked) {
                charsPerLine = charsPerLineWindow;
                vLine.style.display = "none";
                codeWidth.style.fontWeight = "bold";
                codeWidthManual.style.fontWeight = "normal";
            } else {
                vLine.style.left = `${charsPerLine}ch`;
                vLine.style.display = "block";
                codeWidth.style.fontWeight = "normal";
                codeWidthManual.style.fontWeight = "bold";
            }
            updateContents(charsPerLine);
        }

        const observer = new ResizeObserver(() => {
            update();
        });

        observer.observe(codeBlock);

        checkbox.addEventListener('change', () => {
            slider.disabled = !checkbox.checked;
            update();
        });

        slider.addEventListener('input', () => {
            codeWidthManual.textContent = slider.value;
            update();
        });
    }
  
    function getCharWidth(element) {
        // Create a span to measure width of a single character
        const span = document.createElement('span');
        span.textContent = 'M'; // A typical monospace character
        span.style.visibility = 'hidden';
        span.style.font = getComputedStyle(element).font;
        document.body.appendChild(span);
        const width = span.getBoundingClientRect().width;
        document.body.removeChild(span);
        return width;
    }

    function calculateCharsPerLine() {
        const padding = 2 * 10;
        const width = codeBlock.getBoundingClientRect().width - padding;
        const charWidth = getCharWidth(codeBlock);
        const charsPerLine = Math.floor(width / charWidth);
        // console.log(`Width: ${width}px, Char width: ${charWidth}px, Fits: ${charsPerLine} chars`);
        return charsPerLine;
    }

    main();
</script>
