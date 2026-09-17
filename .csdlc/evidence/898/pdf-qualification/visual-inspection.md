# Issue 898 PDF visual inspection

- Renderer: `printpdf@0.12.8; layout=codefriend-pdf-v1`
- Retained PDF: `report.pdf`
- Extracted semantics: `extracted.txt`
- Render command: `pdftoppm -png -r 110 report.pdf pages/page`
- Pages inspected: all five retained page images, `pages/page-1.png` through
  `pages/page-5.png`
- Declared printable width: 174000 micrometers
- Maximum measured rendered line width: 173976 micrometers
- Result: PASS

The inspected output has readable Unicode (`Résumé café π`), glyph-width-bound
wrapping for the long URL and the 80-character wide `W` token, visible
citations, complete remediation and test-plan sections, and a final
output-boundary section. No page shows clipped text, overlap, missing bottom
content, broken glyphs, or content outside the page margins. The five rendered
page count matches the manifest and the extracted text contains the required
source, claim, finding, remediation, test-plan, path, and Unicode semantics.

This is local renderer qualification only. It does not claim provider work,
remote publication, customer hosting, or visual identity with HTML.
