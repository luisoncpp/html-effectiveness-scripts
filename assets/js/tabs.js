(function () {
  document.querySelectorAll('.code-panel').forEach(function (panel) {
    panel.querySelectorAll('.code-panel__tab').forEach(function (btn) {
      btn.addEventListener('click', function () {
        var idx = btn.dataset.tab;
        panel.querySelectorAll('.code-panel__tab').forEach(function (b) { b.classList.remove('code-panel__tab--active'); });
        panel.querySelectorAll('.code-panel__panel').forEach(function (p) { p.classList.remove('code-panel__panel--active'); });
        btn.classList.add('code-panel__tab--active');
        var p = panel.querySelector('.code-panel__panel[data-panel="' + idx + '"]');
        if (p) p.classList.add('code-panel__panel--active');
      });
    });
  });
})();
