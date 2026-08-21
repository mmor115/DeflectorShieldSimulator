/* Two conveniences on the landing page, both optional. The page is complete and correct
   with scripting disabled: the download rows are plain links and the version reads
   "latest release". */
(function () {
    "use strict";

    var script = document.currentScript;
    var repo = (script && script.dataset.repo) || "";

    /* Highlight the row matching the visitor's platform. Nothing is hidden or reordered,
       so a wrong guess costs the reader nothing. */
    var platform = (navigator.userAgent || "") + " " + (navigator.platform || "");

    document.querySelectorAll(".download-list li[data-detect]").forEach(function (li) {
        var token = li.dataset.detect;

        if (token && platform.indexOf(token) !== -1) {
            li.classList.add("suggested");
        }
    });

    /* Fill in the current release tag. On failure the static text stands. */
    var slot = document.querySelector("[data-latest-tag]");
    var match = repo.match(/github\.com\/([^/]+\/[^/]+)/);

    if (!slot || !match) {
        return;
    }

    fetch("https://api.github.com/repos/" + match[1] + "/releases/latest")
        .then(function (r) { return r.ok ? r.json() : null; })
        .catch(function () { return null; })
        .then(function (data) {
            if (data && data.tag_name) {
                slot.textContent = data.tag_name;
            }
        });
})();
