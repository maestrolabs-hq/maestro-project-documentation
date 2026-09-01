/* Progressive enhancements for the Obsidian Score mdBook theme. */
(() => {
  const scriptUrl = document.currentScript?.src;
  const assetRoot = scriptUrl ? new URL(".", scriptUrl) : new URL(".", document.baseURI);
  const reducedMotion = window.matchMedia("(prefers-reduced-motion: reduce)");
  let initialized = false;

  function installWordmark() {
    const title = document.querySelector(".sidebar .menu-title");
    if (!title || title.querySelector(".maestro-sidebar-wordmark")) return;

    const image = document.createElement("img");
    image.className = "maestro-sidebar-wordmark";
    image.src = new URL("../images/maestro-wordmark.svg", assetRoot).href;
    image.alt = "Maestro";
    image.width = 240;
    image.height = 40;
    title.textContent = "";
    title.append(image);
  }

  function installReadingProgress() {
    const progress = document.createElement("div");
    progress.className = "maestro-reading-progress";
    progress.setAttribute("role", "progressbar");
    progress.setAttribute("aria-label", "Reading progress");
    progress.setAttribute("aria-valuemin", "0");
    progress.setAttribute("aria-valuemax", "100");
    progress.setAttribute("aria-valuenow", "0");
    document.body.append(progress);

    let scheduled = false;
    const update = () => {
      const scrolling = document.scrollingElement;
      const available = Math.max(0, scrolling.scrollHeight - window.innerHeight);
      const ratio = available === 0 ? 1 : Math.min(1, Math.max(0, scrolling.scrollTop / available));
      progress.style.transform = `scaleX(${ratio})`;
      progress.setAttribute("aria-valuenow", String(Math.round(ratio * 100)));
      scheduled = false;
    };
    const schedule = () => {
      if (scheduled) return;
      scheduled = true;
      window.requestAnimationFrame(update);
    };

    window.addEventListener("scroll", schedule, { passive: true });
    window.addEventListener("resize", schedule);
    update();
  }

  function navigationLinks() {
    const seen = new Set();
    return Array.from(document.querySelectorAll(".sidebar .chapter a"))
      .map((anchor) => ({ href: anchor.href, title: anchor.textContent.trim() }))
      .filter(({ href, title }) => {
        if (!href || !title || seen.has(href)) return false;
        seen.add(href);
        return true;
      });
  }

  function installCommandPalette() {
    if (!("HTMLDialogElement" in window)) return;

    const links = navigationLinks();
    if (links.length === 0) return;

    const dialog = document.createElement("dialog");
    dialog.className = "maestro-command";
    dialog.setAttribute("aria-labelledby", "maestro-command-title");

    const title = document.createElement("h2");
    title.className = "visually-hidden";
    title.id = "maestro-command-title";
    title.textContent = "Navigate Maestro";

    const header = document.createElement("div");
    header.className = "maestro-command__header";

    const input = document.createElement("input");
    input.className = "maestro-command__input";
    input.type = "search";
    input.placeholder = "Search this book's navigation";
    input.setAttribute("aria-label", "Filter book navigation");
    input.autocomplete = "off";

    const close = document.createElement("button");
    close.className = "maestro-command__close";
    close.type = "button";
    close.setAttribute("aria-label", "Close command palette");
    close.textContent = "×";

    const results = document.createElement("ul");
    results.className = "maestro-command__results";
    results.setAttribute("aria-label", "Book pages");

    const items = links.map(({ href, title: linkTitle }) => {
      const item = document.createElement("li");
      const anchor = document.createElement("a");
      anchor.href = href;
      anchor.textContent = linkTitle;
      anchor.addEventListener("click", () => dialog.close());
      item.append(anchor);
      results.append(item);
      return { anchor, item, text: linkTitle.toLocaleLowerCase() };
    });

    header.append(input, close);
    dialog.append(title, header, results);
    document.body.append(dialog);

    let restoreFocus = null;
    const visibleAnchors = () => items.filter(({ item }) => !item.hidden).map(({ anchor }) => anchor);
    const open = () => {
      restoreFocus = document.activeElement;
      input.value = "";
      for (const { item } of items) item.hidden = false;
      dialog.showModal();
      input.focus();
    };

    const toolbar = document.querySelector("#mdbook-menu-bar .left-buttons");
    if (toolbar) {
      const trigger = document.createElement("button");
      trigger.className = "maestro-command-trigger";
      trigger.type = "button";
      trigger.setAttribute("aria-label", "Open command palette, Control or Command K");
      trigger.innerHTML = "<span aria-hidden=\"true\">⌘K</span>";
      trigger.addEventListener("click", open);
      toolbar.append(trigger);
    }

    input.addEventListener("input", () => {
      const query = input.value.trim().toLocaleLowerCase();
      for (const entry of items) entry.item.hidden = query !== "" && !entry.text.includes(query);
    });
    input.addEventListener("keydown", (event) => {
      if (event.key !== "ArrowDown") return;
      const first = visibleAnchors()[0];
      if (first) {
        event.preventDefault();
        first.focus();
      }
    });
    results.addEventListener("keydown", (event) => {
      if (event.key !== "ArrowDown" && event.key !== "ArrowUp") return;
      const visible = visibleAnchors();
      const current = visible.indexOf(document.activeElement);
      if (current === -1) return;
      event.preventDefault();
      const delta = event.key === "ArrowDown" ? 1 : -1;
      visible[(current + delta + visible.length) % visible.length].focus();
    });
    close.addEventListener("click", () => dialog.close());
    dialog.addEventListener("close", () => {
      if (restoreFocus instanceof HTMLElement) restoreFocus.focus();
    });
    document.addEventListener("keydown", (event) => {
      if (!(event.ctrlKey || event.metaKey) || event.key.toLocaleLowerCase() !== "k") return;
      event.preventDefault();
      if (dialog.open) dialog.close();
      else open();
    });
  }

  function exposeClaimEvidence() {
    const labels = {
      built: "Built",
      "in-progress": "In progress",
      designed: "Designed",
      exploring: "Exploring",
    };

    for (const claim of document.querySelectorAll(".maestro-claim")) {
      if (claim.querySelector(".maestro-claim__evidence")) continue;
      const { source, status, verified } = claim.dataset;
      if (!source || !labels[status] || !verified) continue;

      const evidence = document.createElement("footer");
      evidence.className = "maestro-claim__evidence";

      const badge = document.createElement("span");
      badge.className = "maestro-claim__status";
      badge.dataset.status = status;
      badge.textContent = labels[status];

      const link = document.createElement("a");
      link.href = source;
      link.target = "_blank";
      link.rel = "noreferrer";
      link.textContent = "View immutable evidence ↗";

      const date = document.createElement("time");
      date.dateTime = verified;
      date.textContent = `Verified ${verified}`;

      evidence.append(badge, link, date);
      claim.append(evidence);
    }
  }

  function improveCopyFeedback() {
    const status = document.createElement("span");
    status.className = "maestro-copy-status";
    status.setAttribute("aria-live", "polite");
    document.body.append(status);

    const buttons = document.querySelectorAll(
      ".clip-button, .copy-button, button[aria-label*='Copy'], button[title*='Copy']",
    );
    for (const button of buttons) {
      button.addEventListener("click", () => {
        const oldLabel = button.getAttribute("aria-label") || button.title || "Copy code";
        window.queueMicrotask(() => {
          button.setAttribute("aria-label", "Code copied");
          button.title = "Code copied";
          status.textContent = "Code copied to clipboard";
          window.setTimeout(() => {
            button.setAttribute("aria-label", oldLabel);
            button.title = oldLabel;
          }, reducedMotion.matches ? 0 : 1800);
        });
      });
    }
  }

  function init() {
    if (initialized || document.documentElement.dataset.maestroTheme === "ready") return;
    initialized = true;
    document.documentElement.dataset.maestroTheme = "ready";
    installWordmark();
    installReadingProgress();
    installCommandPalette();
    exposeClaimEvidence();
    improveCopyFeedback();
  }

  window.MaestroTheme = Object.freeze({ init });
  if (document.readyState === "loading") document.addEventListener("DOMContentLoaded", init, { once: true });
  else init();
})();
