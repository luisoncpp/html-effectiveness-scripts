(function () {
  'use strict';

  function activate (root, idx) {
    root.querySelectorAll('.node').forEach(function (x) {
      x.classList.remove('active');
    });
    const node = root.querySelector('.node[data-idx="' + idx + '"]');
    if (node) node.classList.add('active');

    const panel = root.querySelector('aside');
    if (!panel) return;
    const tmpl = root.querySelector(
      'template.flowchart-detail[data-detail-idx="' + idx + '"]'
    );
    if (!tmpl) return;

    panel.innerHTML = '';
    panel.appendChild(tmpl.content.cloneNode(true));
  }

  function initFlowchart (root) {
    const clickable = root.querySelectorAll('.node[data-idx]');
    clickable.forEach(function (node) {
      node.addEventListener('click', function () {
        activate(root, node.dataset.idx);
      });
    });

    if (clickable.length > 0) {
      activate(root, clickable[0].dataset.idx);
    }
  }

  document.querySelectorAll('.flowchart-sheet').forEach(initFlowchart);
})();
