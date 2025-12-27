# Task Completion Checklist

When completing any coding task in this project, follow these steps:

## 1. Code Quality Checks
Run the automated fix script to ensure code quality:
```bash
./fix-all.sh
```
This will:
- Auto-fix clippy warnings
- Apply proper formatting

## 2. Verify Changes
Manually review the changes to ensure they are correct.

## 3. Run Full Test Suite
Before committing, run the comprehensive check script:
```bash
./check-all.sh
```
This will verify:
- All tests pass (`cargo nextest run --workspace`)
- WASM build succeeds (`cd sres_egui && trunk build`)
- No clippy warnings (`cargo clippy --workspace`)
- Code is properly formatted (`cargo fmt --check`)

## 4. Alternative: Individual Commands
If you prefer to run checks individually:
```bash
# 1. Auto-fix issues
cargo clippy --fix --allow-dirty
cargo fmt

# 2. Run tests
cargo nextest run

# 3. Verify WASM build (if frontend was modified)
cd sres_egui && trunk build && cd ..

# 4. Final verification
cargo clippy --workspace
cargo fmt --check
```

## 5. Commit Changes
Once all checks pass, commit the changes:
```bash
git add .
git commit -m "Description of changes"
```

## Important Notes
- Never commit code with clippy warnings
- Never commit unformatted code
- Always ensure tests pass before committing
- If working on frontend (sres_egui), verify WASM build succeeds
- Use `./check-all.sh` as a pre-commit verification step
