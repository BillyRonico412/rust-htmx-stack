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

| Layer                    | Techno / Crate                 | Rôle                                                                       |
| :----------------------- | :----------------------------- | :------------------------------------------------------------------------- |
| **Backend Framework**    | **Axum**                       | Framework HTTP async basé sur Tokio, moderne et composable.                |
| **Database & ORM**       | **SQLx**                       | Client SQL async avec vérification des requêtes et types à la compilation. |
| **Auth**                 | **`oauth2` + `openidconnect`** | Authentification sociale via Google OAuth2.                                |
| **Templating**           | **Askama**                     | Moteur de templates HTML compilé en code Rust (sécurité & vitesse).        |
| **Server Interactivity** | **htmx**                       | Mises à jour dynamiques du DOM via des fragments HTML serveur.             |
| **Client Interactivity** | **Alpine.js**                  | Micro-interactions locales UI (modales, toggles, copie de lien).           |
| **Design / UI**          | **DaisyUI**                    | Composants CSS-only basés sur Tailwind CSS.                                |
| **IA Orchestration**     | **`rig-core` (RIG)**           | Framework Rust pour interagir avec les LLMs (résumés, analyse).            |
| **Utilitaires**          | **`fast_qr`**                  | Génération ultra-rapide de QR Codes en SVG/PNG côté serveur.               |

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
* [x] Définir un `AppState` partagé et thread-safe (`Arc<Mutex<T>>`).
* [x] **Exo (CRUD) :** Créer un gestionnaire de tâches (Todos) en mémoire dans `AppState`.

---

### ⚡ Phase 3 : Premier Projet HTMX - Todo List
> **Objectif :** Transformer le gestionnaire de tâches en premier projet full stack Rust + HTMX.

* [ ] Rendre les pages et fragments HTML côté serveur avec `askama`.
* [ ] Afficher, créer, compléter et supprimer une tâche avec des requêtes htmx, sans rechargement complet de la page.
* [ ] Découper l'interface en fragments : liste, formulaire de création et compteur de tâches.
* [ ] Ajouter Alpine.js et DaisyUI dès ce projet pour installer les bases du workflow frontend.
* [ ] **Exo 1 - Charger plus :** ajouter une liste paginée de tâches terminées ou archivées. Le bouton « Charger la suite » utilise `hx-get`, cible son propre emplacement et `hx-swap="outerHTML"` pour être remplacé par les éléments suivants et un nouveau bouton.
* [ ] **Exo 2 - Recherche dynamique :** filtrer les tâches avec un champ utilisant `hx-trigger="keyup changed delay:500ms"`. Afficher un spinner avec `hx-indicator` pendant que le serveur renvoie le fragment de résultats.
* [ ] **Exo 3 - Édition en ligne :** rendre le titre et la description d'une tâche modifiables. Un `hx-get` remplace la tâche par son formulaire pré-rempli ; un `hx-put` renvoie ensuite le fragment en lecture seule dans le même `hx-target`.
* [ ] **Exo 4 - Validation à la volée :** vérifier le titre d'une nouvelle tâche côté serveur avec `hx-post`, `hx-trigger="change"` et `hx-target="this"`. Renvoyer un message d'erreur si le titre est vide ou déjà utilisé, sinon une confirmation.
* [ ] **Exo 5 - Suppression confirmée :** supprimer une tâche via `hx-delete` avec `hx-confirm` et `hx-target="closest li"` (ou `closest tr` pour une vue tableau). Ajouter une transition CSS de sortie pour que l'élément disparaisse en fondu.
* [ ] Utiliser les en-têtes htmx (`HX-Request`, `HX-Trigger`) lorsque cela apporte une meilleure expérience, notamment pour mettre à jour le compteur après une mutation.

---

### 🎨 Phase 4 : Mini-projets Alpine.js + DaisyUI
> **Objectif :** Ancrer Alpine.js et DaisyUI à travers de petits projets avec une logique serveur Axum minimale.

* [ ] Maîtriser les directives Alpine : `x-data`, `x-show`, `x-on`, `x-model`, `x-bind`, `x-transition` et `x-for`.
* [ ] Utiliser DaisyUI pour les formulaires, boutons, cartes, modales, alertes et thèmes, sans écrire de composants visuels from scratch.
* [ ] **Exo 1 - Pomodoro :** choisir une durée, démarrer / mettre en pause / réinitialiser le minuteur avec Alpine ; persister les sessions terminées via une route Axum.
* [ ] **Exo 2 - Générateur de liens de partage :** ouvrir une modale DaisyUI, valider le formulaire en Alpine, demander au serveur de générer un slug et copier le lien avec un feedback visuel.
* [ ] **Exo 3 - Tableau de sondage :** créer un sondage simple côté serveur et voter via htmx ; utiliser Alpine pour les états locaux, confirmations et l'affichage conditionnel des résultats.
* [ ] Réutiliser une petite base de composants : modal, toast, bouton de copie, formulaire validé et état de chargement.

---

### 🔐 Phase 5 : BDD, Authentification & Sessions - Carnet de Lecture
> **Objectif :** Construire une application simple qui rend nécessaires une BDD relationnelle, l'authentification et les sessions.

* [ ] Créer un carnet de lecture : un utilisateur connecté gère ses livres et les classe dans des listes telles que « À lire », « En cours » et « Terminé ».
* [ ] Configurer `sqlx` avec SQLite, les migrations `.sql` et des requêtes vérifiées à la compilation lorsque possible.
* [ ] Modéliser deux relations : `users` possède plusieurs `books`, et `books` appartient à une `reading_list`.
* [ ] Mettre en place une inscription / connexion par email et mot de passe avec hashage sécurisé, puis une session via `tower-sessions`.
* [ ] Créer l'extracteur Axum `AuthenticatedUser` pour protéger le carnet personnel et vérifier que chaque utilisateur ne manipule que ses propres livres.
* [ ] Garder Google OAuth2 comme amélioration optionnelle après le flux local email / mot de passe.

---

## 🏆 Projet de Validation : ROTI (Return On Time Invested)

Une application permettant d'évaluer rapidement l'efficacité des réunions, conférences ou ateliers.

1. Admin se connecte via Google
2. Crée un événement "Workshop WebGPU"
3. Génère un QR Code + Lien unique anonyme
4. Participants scannent et notent de 1 à 5 + commentaire
5. Dashboard Admin : Métriques temps réel
