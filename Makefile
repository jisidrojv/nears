# NEARS — Makefile
# Comandos de desarrollo y build

include .env
export

TAILWIND = ./tailwindcss-linux-x64
CSS_INPUT = app/global.css
CSS_OUTPUT = public/style.css

# ─── Desarrollo ───
.PHONY: dev css css-watch build clean

# Compilar CSS y ejecutar servidor
dev: css
	cargo run

# Compilar CSS una vez
css:
	$(TAILWIND) -i $(CSS_INPUT) -o $(CSS_OUTPUT)

# Compilar CSS en modo watch (en otra terminal)
css-watch:
	$(TAILWIND) -i $(CSS_INPUT) -o $(CSS_OUTPUT) --watch

# ─── Producción ───

# Build completo de producción
build: css-prod
	cargo build --release

# Compilar CSS minificado para producción
css-prod:
	$(TAILWIND) -i $(CSS_INPUT) -o $(CSS_OUTPUT) --minify

# ─── Limpieza ───

clean:
	cargo clean
	rm -f $(CSS_OUTPUT)

# ─── Database ───
db-reset:
	@echo "🔥 Reseteando la base de datos..."
	@psql $(DATABASE_URL) -c "DROP SCHEMA public CASCADE; CREATE SCHEMA public;"
	@echo "🌱 Corriendo el SEED de Rust..."
	@cargo run --bin seed

