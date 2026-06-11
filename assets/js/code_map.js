(function () {
    function resolveRef(canvas, ref) {
        if (!ref) return null;
        var dot = ref.indexOf(".");
        if (dot > -1) {
            var anchor = canvas.querySelector('[data-anchor="' + ref + '"]');
            if (anchor) return anchor;
            return canvas.querySelector('[data-card="' + ref.slice(0, dot) + '"]');
        }
        return canvas.querySelector('[data-card="' + ref + '"]');
    }

    function drawCanvas(canvas) {
        var svg = canvas.querySelector(".code-map__arrows");
        if (!svg) return;
        var base = canvas.getBoundingClientRect();

        svg.querySelectorAll(".code-map__arrow").forEach(function (path) {
            var fromEl = resolveRef(canvas, path.dataset.from);
            var toEl = resolveRef(canvas, path.dataset.to);
            if (!fromEl || !toEl) {
                path.removeAttribute("d");
                return;
            }
            var a = fromEl.getBoundingClientRect();
            var b = toEl.getBoundingClientRect();
            var sy = a.top + a.height / 2 - base.top;
            var ty = b.top + b.height / 2 - base.top;
            var sx, tx;
            if (b.left >= a.right) {
                sx = a.right - base.left + 4;
                tx = b.left - base.left - 3;
            } else if (b.right <= a.left) {
                sx = a.left - base.left - 4;
                tx = b.right - base.left + 3;
            } else {
                sx = a.right - base.left + 4;
                tx = b.right - base.left + 3;
            }
            var dx = Math.max(Math.abs(tx - sx) / 2, 36);
            var dir = tx >= sx ? 1 : -1;
            var d = "M" + sx + "," + sy +
                " C" + (sx + dir * dx) + "," + sy +
                " " + (tx - dir * dx) + "," + ty +
                " " + tx + "," + ty;
            path.setAttribute("d", d);
        });
    }

    function drawAll() {
        document.querySelectorAll("[data-code-map]").forEach(drawCanvas);
    }

    if (document.readyState === "loading") {
        document.addEventListener("DOMContentLoaded", drawAll);
    } else {
        drawAll();
    }
    window.addEventListener("resize", drawAll);
})();
