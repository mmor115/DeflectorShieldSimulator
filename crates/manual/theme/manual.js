/* Substring search over the manual. The index is small enough that a linear scan is
   instant, so there is no ranking library and no build step. The page works without
   this script; only the search box stops responding. */
(function () {
    "use strict";

    var script = document.currentScript;
    var base = (script && script.dataset.base) || "";
    var input = document.getElementById("q");
    var results = document.getElementById("results");

    if (!input || !results) {
        return;
    }

    var index = null;

    fetch(base + "/manual/search-index.json")
        .then(function (r) { return r.ok ? r.json() : null; })
        .catch(function () { return null; })
        .then(function (data) { index = data; });

    function escapeHtml(s) {
        return s.replace(/[&<>"]/g, function (c) {
            return { "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;" }[c];
        });
    }

    function search(query) {
        var needle = query.toLowerCase();
        var hits = [];

        index.forEach(function (chapter) {
            if (chapter.title.toLowerCase().indexOf(needle) !== -1) {
                hits.push({
                    href: base + "/manual/" + chapter.slug + ".html",
                    label: chapter.title,
                    where: chapter.part
                });
            }

            chapter.headings.forEach(function (heading) {
                if (heading.text.toLowerCase().indexOf(needle) !== -1) {
                    hits.push({
                        href: base + "/manual/" + chapter.slug + ".html#" + heading.id,
                        label: heading.text,
                        where: chapter.title
                    });
                }
            });

            var at = chapter.text.toLowerCase().indexOf(needle);

            if (at !== -1) {
                hits.push({
                    href: base + "/manual/" + chapter.slug + ".html",
                    label: "…" + chapter.text.slice(Math.max(0, at - 40), at + 80) + "…",
                    where: chapter.title
                });
            }
        });

        return hits.slice(0, 20);
    }

    input.addEventListener("input", function () {
        var query = input.value.trim();

        if (!index || query.length < 2) {
            results.hidden = true;
            results.innerHTML = "";
            return;
        }

        var hits = search(query);

        results.hidden = false;
        results.innerHTML = hits.length
            ? hits.map(function (h) {
                return '<li><a href="' + h.href + '">' + escapeHtml(h.label) + "</a> " +
                       '<span class="where">' + escapeHtml(h.where) + "</span></li>";
            }).join("")
            : "<li>No matches.</li>";
    });
})();
