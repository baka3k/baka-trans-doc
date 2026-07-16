# Local Document Translation Tool Specification

## Project Name

**Local Document Translator**

---

# 1. Objective

Develop a cross-platform desktop application using **Tauri** that translates Microsoft Office documents locally through **Ollama** while preserving the original document formatting.

The application must support:

* DOCX
* PPTX

The translated document should look as close as possible to the original.

---

# 2. Goals

### Functional Goals

* Translate Japanese → Vietnamese (initial version)
* Support local Ollama models
* Preserve document formatting
* Preserve images
* Preserve tables
* Preserve layout
* Preserve fonts where possible
* Preserve colors
* Preserve bullet lists
* Preserve numbering

### Non-functional Goals

* Fully offline
* No cloud services
* Cross-platform (Windows first)
* Responsive UI
* Translation progress indicator
* Recoverable if translation fails

---

# 3. Architecture

```
                +----------------------+
                |      Tauri UI        |
                +----------+-----------+
                           |
                    invoke()
                           |
                +----------v-----------+
                |    Rust Backend      |
                |                      |
                | Document Parser      |
                | Translation Engine   |
                | Formatter            |
                | Exporter             |
                +----------+-----------+
                           |
                           |
                  HTTP localhost:11434
                           |
                           ▼
                        Ollama
```

---

# 4. Supported Models

The application shall support any Ollama model.

Examples:

* gemma3
* qwen3
* llama3
* mistral

The model name shall be configurable.

---

# 5. Supported Input Formats

## DOCX

Read:

* Paragraphs
* Runs
* Tables
* Headers
* Footers
* Text boxes
* Lists
* Hyperlinks

Ignore:

* Comments
* Track Changes
* Macros

---

## PPTX

Read:

* Slide titles
* Text boxes
* Shapes
* Tables
* SmartArt (text only if accessible)
* Speaker Notes (optional)

Ignore:

* Animations
* Embedded videos

---

# 6. Output

The application shall generate

```
original.docx

↓

original_vi.docx
```

or

```
presentation.pptx

↓

presentation_vi.pptx
```

---

# 7. Translation Workflow

```
Open document

↓

Extract text blocks

↓

Split into translation units

↓

Translate

↓

Replace translated text

↓

Save document
```

---

# 8. Translation Unit

A translation unit should be the smallest meaningful block while preserving formatting.

Examples

DOCX

* paragraph
* table cell
* header
* footer

PPTX

* text box
* shape text
* table cell
* title

The application must not translate character-by-character or individual formatting runs unless necessary.

---

# 9. Formatting Preservation

The application must preserve

* Font family
* Font size
* Bold
* Italic
* Underline
* Text color
* Highlight
* Alignment
* Paragraph spacing
* Line spacing
* Bullet lists
* Numbered lists
* Tables
* Images
* Hyperlinks
* Page breaks
* Slide layout
* Theme colors

---

# 10. Translation Pipeline

```
Extract text

↓

Normalize

↓

Chunk

↓

Ollama

↓

Receive translation

↓

Apply translation

↓

Save
```

---

# 11. Chunk Strategy

Large documents should never be translated in one request.

Chunk size should be configurable.

Default:

* 1,500–2,000 characters

Rules:

* Never split inside a sentence when possible.
* Never split inside a table cell.
* Never split inside a text box.
* Preserve paragraph boundaries whenever possible.

---

# 12. Prompt

System Prompt

```
You are a professional Japanese to Vietnamese translator.

Rules:

- Translate naturally.
- Preserve paragraph boundaries.
- Do not explain.
- Do not summarize.
- Do not translate URLs.
- Do not translate code.
- Preserve placeholders.
- Return only the translated text.
```

---

# 13. Placeholder Protection

Before translation, replace special content with placeholders.

Examples

```
{{IMAGE_1}}

{{URL_2}}

{{CODE_3}}

{{NUMBER_4}}
```

Restore after translation.

---

# 14. Error Handling

If one chunk fails

```
Retry

↓

Still fail

↓

Skip

↓

Continue
```

At the end produce a report

```
Chunk 14 failed
Chunk 28 failed
```

---

# 15. User Interface

Main Window

```
---------------------------------------

Input File

[ Browse ]

Model

[ gemma3 ▼ ]

Source Language

[ Japanese ▼ ]

Target Language

[ Vietnamese ▼ ]

Output Folder

[ Browse ]

---------------------------------------

Translate

---------------------------------------

Progress

█████████████░░░░░░

35%

Current:

Slide 15

---------------------------------------

Log

...
```

---

# 16. Progress Reporting

Progress should display

* Current page
* Current slide
* Current paragraph
* Percentage
* Estimated remaining time

---

# 17. Project Structure

```
src-tauri/

    commands/
        translate.rs

    parser/
        docx.rs
        pptx.rs

    translator/
        ollama.rs
        chunk.rs
        prompt.rs

    formatter/
        replace.rs

    exporter/
        save.rs

    progress/

    utils/

src/

    pages/

    components/

    store/
```

---

# 18. Future Enhancements

* PDF support
* XLSX support
* Markdown support
* Batch translation
* Translation memory
* Glossary / terminology management
* Bilingual output
* AI-powered document quality review
* Streaming translation preview
* OCR for scanned PDFs
