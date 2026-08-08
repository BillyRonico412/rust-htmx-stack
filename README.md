# 🚀 Roadmap : Stack Rust + HTMX

Mon plan d'action incrémental pour apprendre et maîtriser une nouvelle stack web ultra-légère, réactive et performante : **Rust (Axum + SQLx)** côté backend, **HTMX + Alpine.js + DaisyUI** côté frontend, et **RIG** pour l'intégration IA.

---

## 💡 Pourquoi ce choix d'architecture ?

### 5 ans bloqué dans l'écosysteme React, Typescipt.

Cela fait 5 ans que je fais du **React**. Avec le temps, j'ai pu affiner mon expertise et greffer des outils devenus essentiels dans mon quotidien comme **ShadcnUI**, **Jotai**, tout les tools **TanStack**.

Pour le back, j'utilisais un serveur Node.js basique avec **tRPC**, **better-auth** et **Drizzle ORM** pour avoir le typage de bout en bout côté client et serveur. Ça fonctionnait parfaitement, et pendant longtemps c'était la stack parfaite pour moi.

### L'intégration de Rust dans mon workflow

Mais ces derniers temps, je voulais faire autre chose. J'ai commencé à m'intéresser à **Rust** nottanment pour comprendre le bas niveau. Je suis tombé amoureux du language, de sa philosophie, de son éxpréssivité.

Le problème, c'est que j'avais du mal à l'intégrer à mon workflow. Je me demandais donc : *comment je pourrais continuer à construire des applications tout en ajoutant Rust petit à petit dans mon écosystème, tout en ne faisant pas de projet laboratoire dans un terminal ?*

### La révélation HTMX

La philosophie de **HTMX** a été une vraie révélation pour moi. 

Un serveur retourne du HTML. HTMX qui met à jour uniquement un bout du DOM est une idée qui paraît simple de prime abord, mais qui chamboule tout. 

Au début du Web avec PHP, on essayait déjà de faire la même chose. Mais recharger systématiquement la page complète à chaque action n'était pas une bonne idée. HTMX résout exactement ce problème historique.

Je me rends compte que pour **90% des applications web**, cette approche est largement suffisante. C'est un peu ce qu'on essaie de faire aujourd'hui avec du Next.js ou du Astro, mais avec **beaucoup moins de complexité**.

C'est pour moi une porte de sortie du monde de JavaScript pour pouvoir continuer à faire des applications web réactives et performantes tout en utilisant un autre langage. 

Du **vrai full stack ultra léger**.

---

## 🛠️ La Stack Technique

| Layer | Techno / Crate | Rôle |
| :--- | :--- | :--- |
| **Backend Framework** | **Axum** | Framework HTTP async basé sur Tokio, moderne et composable. |
| **Database & ORM** | **SQLx** | Client SQL async avec vérification des requêtes et types à la compilation. |
| **Auth** | **`oauth2` + `openidconnect`** | Authentification sociale via Google OAuth2. |
| **Templating** | **Askama** | Moteur de templates HTML compilé en code Rust (sécurité & vitesse). |
| **Server Interactivity** | **htmx** | Mises à jour dynamiques du DOM via des fragments HTML serveur. |
| **Client Interactivity** | **Alpine.js** | Micro-interactions locales UI (modales, toggles, copie de lien). |
| **Design / UI** | **DaisyUI** | Composants CSS-only basés sur Tailwind CSS. |
| **IA Orchestration** | **`rig-core` (RIG)** | Framework Rust pour interagir avec les LLMs (résumés, analyse). |
| **Utilitaires** | **`fast_qr`** | Génération ultra-rapide de QR Codes en SVG/PNG côté serveur. |

---

## 📅 Plan d'Action Incrémental (5 Phases)

### 🏁 Phase 1 : L'Asynchronisme en Rust (`tokio`)
> **Objectif :** Dompter le modèle d'exécution asynchrone de Rust.

* [x] Comprendre le fonctionnement d'une `Future`, du `.await` et de l'Event Loop Tokio.
* [x] **Exo 1 :** Écrire un script async simulant des délais avec `tokio::time::sleep`.
* [x] **Exo 2 :** Créer un scraper HTTP concurrent avec `reqwest` et `tokio::spawn` / `join_all`.

---

### 🚀 Phase 2 : Premier Serveur HTTP avec Axum
> **Objectif :** Valider la gestion des routes, des extracteurs et de l'état partagé.

* [x] Comprendre le routing Axum (`GET`, `POST`), les extracteurs (`Path`, `Query`, `Json`) et les réponses.
* [x] Définir un `AppState` partagé et thread-safe (`Arc<RwLock<T>>` ou Pool BDD).
* [x] **Exo 1 :** Créer une API basique (`/ping` et `/api/status` avec `serde`).
* [x] **Exo 2 (CRUD) :** Créer un gestionnaire de tâches (Todos) en mémoire dans `AppState`.

---

### 🔐 Phase 3 : Base de Données, OAuth2 Google & Sessions
> **Objectif :** Connecter SQLx et implémenter le flux "Login with Google".

* [ ] Configurer `sqlx` avec SQLite/PostgreSQL et gérer les migrations `.sql`.
* [ ] Configurer la console Google Cloud (OAuth 2.0 Client ID & Secret).
* [ ] Utiliser le crate `oauth2` pour gérer la redirection vers Google et le callback `/auth/google/callback`.
* [ ] Stocker le profil utilisateur en BDD et créer une session sécurisée via `tower-sessions` ou `tower-cookies`.
* [ ] Créer l'extracteur Axum `AuthenticatedUser` pour sécuriser les routes du dashboard.

---

### ⚡ Phase 4 : Frontend & Interactivity (Alpine.js + DaisyUI)
> **Objectif :** Créer des interfaces modernes et gérer l'état local client.

* [ ] Maîtriser les directives Alpine : `x-data`, `x-show`, `x-on`, `x-model`.
* [ ] **Exo 1 :** Créer un composant Modale DaisyUI piloté par Alpine.
* [ ] **Exo 2 :** Développer un bouton "Copier dans le presse-papier" avec feedback visuel réactif.
* [ ] **Exo 3 :** Intégrer DaisyUI (cartes, jauges de score, formulaires, thèmes).

---

### 🎯 Phase 5 : HTMX, Moteur de Templates & Intégration RIG (IA)
> **Objectif :** Connecter htmx pour l'interactivité serveur et RIG pour l'analyse IA.

* [ ] Utiliser `askama` pour le rendu des templates HTML vérifiés à la compilation.
* [ ] Utiliser htmx pour la soumission de vote anonyme sans rechargement de page.
* [ ] Configurer le crate `rig-core` avec une clé d'API (OpenAI / Anthropic / Ollama local).
* [ ] **Exo :** Passer une liste de commentaires textuels à RIG et afficher le résumé généré sous forme de fragment HTML via htmx (`hx-get="/events/:id/ai-summary"`).

---

## 🏆 Projet de Validation : ROTI (Return On Time Invested)

Une application permettant d'évaluer rapidement l'efficacité des réunions, conférences ou ateliers.

1. Admin se connecte via Google
2. Crée un événement "Workshop WebGPU"
3. Génère un QR Code + Lien unique anonyme
4. Participants scannent et notent de 1 à 5 + commentaire
5. Dashboard Admin : Métriques temps réel + Résumé IA
