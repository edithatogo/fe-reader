# Scanning, OCR and Ingestion Plan

## Scope

Scanning/OCR is not the first engine milestone, but it is an important workflow surface.

## Ingestion sources

- desktop file picker
- mobile document picker
- drag/drop
- browser download handoff
- virtual printer later
- scanner APIs later
- camera capture on mobile later

## Scanner integrations

| Platform | Candidate path |
|---|---|
| Windows | WIA/TWAIN bridge later |
| macOS | ImageCaptureCore later |
| Linux | SANE bridge later |
| Android | camera/document scanner intent where available |
| iOS | VisionKit document scanner path later |

## OCR policy

- MVP may use external/provider OCR contracts.
- Later OCR providers can include Tesseract or platform OCR APIs.
- OCR results must carry confidence and bounding boxes.
- OCR-derived text must be clearly distinguishable from native PDF text.
- Redaction verification may optionally OCR rendered pages to detect visual residual text.
