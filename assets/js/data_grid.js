(function () {
  'use strict';

  var map = {
    'High': 'high', 'HIGH': 'high',
    'Med': 'med', 'MED': 'med',
    'Medium': 'medium', 'MEDIUM': 'medium',
    'Low': 'low', 'LOW': 'low'
  };

  document.querySelectorAll('.data-grid tbody td').forEach(function (td) {
    var key = td.textContent.trim();
    var cls = map[key];
    if (cls && td.innerHTML.trim() === key) {
      td.innerHTML = '<span class="badge badge--' + cls + '">' + key + '</span>';
    }
  });
})();
