

# TypForge

A simple tool for creating personalised and application-specific cover letter *templates* in <a href="https://typst.app/">Typst</a> using generative AI.

TypForge is *not* meant for creating cover letters for you, instead it helps create basic templates while adding content suggestions in the source code comments. This helps you target specific points based on the application specification, given your past experience and relevant education.

Currently, TypForge uses Google's Gemini 2.0 Flash API (because it's essentially free and has near-SOTA performance) - but the `main.rs` file can easily be tweaked to utilise any generative AI API.


Honestly, I made this because I thought it would be more fun than just using basic pre-made templates, and I was right :)


## Using the Tool:

After finding a job/application that requires a cover letter:
1. Paste contents of resume/CV into the `CV content` text input
2. Copy and paste contents/description from the application into the `Application details` text input
3. Click \[Generate Template\]
4. After generating, you'll get the Typst source code and a rendered PDF of the cover letter
5. Updating the contents of the source code textbox will recompiled and re-render the PDF.
6. When you're done, you can download the PDF directly from the preview window.



## Local Installation:

TypForge can easily be run on your local device, so long as you have an up-to-date Rust installation and a generative AI API key. As stated previously, by default TypForge looks for the environment variable `GEMINI_API_KEY`, but this can easily be changed.
If you want to change which model is being queried, be sure to check the response parsing and content extraction works for the returned JSON schema (this code can be found in the `create_template` route handler). Also, the template prompt can easily be edited to match your 
preferences - it's stored in the `prompt.txt` file.


## Further Extension/Development Plans:

- [ ] Clean up the UI a bit
- [ ] Better embedded Typst source code editor
- [ ] Nicer PDF preview window
- [ ] Refine prompt
- [ ] See if I can work around the Typst compiler to remove the font file dependency.
- [ ] Start hosting TypForge

