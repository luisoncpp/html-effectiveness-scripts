(function () {
  'use strict';

  function initFlowchart (root) {
    const nodes = root.querySelectorAll('.node');
    const panel = root.querySelector('aside');
    if (!panel) return;

    const titleEl = panel.querySelector('#p-title');
    const metaEl = panel.querySelector('#p-meta');
    const bodyEl = panel.querySelector('#p-body');
    const codeEl = panel.querySelector('#p-code');
    const hintEl = panel.querySelector('.hint');

    const DETAIL = window.DETAIL || {};

    function activate (node) {
      nodes.forEach(function (x) { x.classList.remove('active'); });
      node.classList.add('active');
      const key = node.dataset.key;
      if (!key) return;
      const d = DETAIL[key];
      if (!d) return;
      if (hintEl) hintEl.style.display = 'none';
      if (titleEl) titleEl.innerHTML = d.title;
      if (metaEl) metaEl.textContent = d.meta;
      if (bodyEl) bodyEl.innerHTML = d.body;
      if (codeEl) {
        if (d.code) {
          codeEl.style.display = 'block';
          codeEl.textContent = d.code;
        } else {
          codeEl.style.display = 'none';
        }
      }
    }

    nodes.forEach(function (n) {
      n.addEventListener('click', function () {
        activate(n);
      });
    });

    // Activate first node by default if present
    if (nodes.length > 0) {
      activate(nodes[0]);
    }
  }

  document.querySelectorAll('.flowchart-sheet').forEach(initFlowchart);
})();
