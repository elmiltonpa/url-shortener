# 🔗 Pro-URL Shortener: Enterprise-Grade Link Management

Este es un acortador de URLs de alto rendimiento diseñado con un enfoque en la **seguridad, la escalabilidad y la observabilidad**. Construido con un backend robusto en **Rust** y un frontend optimizado en **Astro 5 + Vue 3**, el sistema no solo acorta enlaces, sino que proporciona analíticas detalladas y protección activa contra amenazas.

---

## 🛠️ Stack Tecnológico

### Backend (The Engine)
- **Lenguaje:** [Rust](https://www.rust-lang.org/) (Safety & Performance).
- **Web Framework:** [Axum](https://github.com/tokio-rs/axum) (basado en Tokio, arquitectura asíncrona).
- **Base de Datos:** [PostgreSQL](https://www.postgresql.org/) con [SQLx](https://github.com/launchbadge/sqlx) (Type-safe, compile-time checked queries).
- **Seguridad:** 
    - Integración con **Google Safe Browsing API** para detección de malware/phishing.
    - Implementación de **Rate Limiting** y **CORS** granular.
    - Autenticación JWT y Google OAuth.

### Frontend (The Interface)
- **Framework Principal:** [Astro 5](https://astro.build/) (Islands Architecture para máxima velocidad de carga).
- **UI Framework:** [Vue 3](https://vuejs.org/) (Composition API) para componentes reactivos complejos.
- **Styling:** [Tailwind CSS 4.0](https://tailwindcss.com/) con estética **Glassmorphism**.
- **Estado:** Composables personalizados para lógica desacoplada de la UI.

---

## 🚀 Características Destacadas

### 1. Arquitectura de Alto Rendimiento
El backend en Rust garantiza latencias mínimas en el redireccionamiento (sub-ms), esencial para sistemas de alto tráfico. El uso de Astro en el frontend minimiza el envío de JavaScript innecesario al cliente mediante su arquitectura de islas.

### 2. Analíticas en Tiempo Real
Dashboard avanzado que visualiza:
- Conteo de clics preciso (BigInt optimizado).
- Referrers y métricas de usuario.
- Historial detallado por enlace.

### 3. Seguridad Proactiva
Antes de crear un enlace, el sistema consulta los servicios de **Google Safe Browsing**. Si la URL de destino es marcada como peligrosa, la creación se bloquea automáticamente, protegiendo a los usuarios finales de phishing o malware.

### 4. DX (Developer Experience)
- **Clean Architecture:** Separación clara entre `Handlers`, `Services`, `Repositories` y `Models`.
- **Migraciones Controladas:** Gestión de base de datos versionada mediante SQLx.
- **Type Safety:** Contratos de tipos compartidos conceptualmente entre el backend (Rust Structs) y el frontend (TS Interfaces).

---

## 🏗️ Estructura del Proyecto

```text
/BACKEND
├── src/
│   ├── handlers/    # Controladores de entrada (REST)
│   ├── services/    # Lógica de negocio (Validación, Safe Browsing)
│   ├── repository/  # Abstracción de acceso a datos (SQLx)
│   ├── models/      # Entidades de dominio y DTOs
│   └── middleware/  # Auth, Rate limiting, Logging
└── migrations/      # Evolución del esquema de base de datos

/FRONTEND
├── src/
│   ├── components/  # Arquitectura de Islas (Astro + Vue)
│   ├── composables/ # Lógica de estado reutilizable
│   ├── actions/     # Integración con Astro Actions para llamadas al API
│   └── layouts/     # Estructuras de diseño consistentes
```

---

## 🛠️ Configuración Rápida

### Requisitos
- Rust (Latest Stable)
- Node.js (v20+) & pnpm
- PostgreSQL

### Backend
1. Navegar a `/BACKEND`.
2. Copiar `.env.example` a `.env` y configurar credenciales.
3. Ejecutar migraciones: `sqlx migrate run`.
4. Iniciar: `cargo run`.

### Frontend
1. Navegar a `/FRONTEND`.
2. Instalar dependencias: `pnpm install`.
3. Iniciar entorno de desarrollo: `pnpm dev`.

---

## 📈 Próximos Pasos (Roadmap)
- [ ] Generación automática de códigos QR para cada link.
- [ ] Exportación de reportes de analíticas en PDF/CSV.
- [ ] Despliegue mediante Docker & Kubernetes.

---

**Desarrollado con ❤️ para demostrar capacidades en Fullstack Engineering de alta performance.**
