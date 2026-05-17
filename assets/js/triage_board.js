// ---------- data ----------
var COLUMNS = [
  { key: 'now',   label: 'Now',   rationale: 'Blocking the v2.4 release or actively losing user data.' },
  { key: 'next',  label: 'Next',  rationale: 'High-leverage and well-scoped — start the moment Now clears.' },
  { key: 'later', label: 'Later', rationale: 'Real, but can ride to a future cycle without anyone noticing.' },
  { key: 'cut',   label: 'Cut',   rationale: 'Not this quarter — close, dedupe, or push back to the requester.' }
];

var EST_PTS = { S: 1, M: 2, L: 3 };

// tag, est, owner, col
var INITIAL = [
  { id: 'BIR-241', title: 'Fix sync conflict toast firing twice on reconnect', tag: 'bug',   est: 'M', owner: 'AK', col: 'now' },
  { id: 'BIR-238', title: 'Comments lost when editing offline then reloading', tag: 'bug',   est: 'L', owner: 'JM', col: 'now' },
  { id: 'BIR-252', title: 'Billing webhook 500s on annual → monthly downgrade', tag: 'bug',   est: 'M', owner: 'RS', col: 'now' },
  { id: 'BIR-219', title: 'Migrate workspace permissions to new ACL table',    tag: 'debt',  est: 'L', owner: 'AK', col: 'now' },
  { id: 'BIR-260', title: 'SSO login loop on Safari 17 with strict cookies',   tag: 'bug',   est: 'S', owner: 'TN', col: 'now' },

  { id: 'BIR-244', title: 'Inline @-mention picker in doc comments',           tag: 'feat',  est: 'M', owner: 'JM', col: 'next' },
  { id: 'BIR-231', title: 'Bulk-archive completed projects from the sidebar',  tag: 'feat',  est: 'S', owner: 'EL', col: 'next' },
  { id: 'BIR-247', title: 'Activity feed: collapse repeated edit events',      tag: 'feat',  est: 'M', owner: 'TN', col: 'next' },
  { id: 'BIR-228', title: 'Notification preferences per project',              tag: 'feat',  est: 'L', owner: 'EL', col: 'next' },
  { id: 'BIR-255', title: 'Keyboard shortcut cheat-sheet overlay (⌘/)',        tag: 'feat',  est: 'S', owner: 'RS', col: 'next' },
  { id: 'BIR-236', title: 'Empty-state illustrations for new workspaces',      tag: 'chore', est: 'S', owner: 'EL', col: 'next' },
  { id: 'BIR-249', title: 'Audit log export to CSV for admins',                tag: 'feat',  est: 'M', owner: 'AK', col: 'next' },

  { id: 'BIR-213', title: 'Dark mode pass on settings + billing pages',        tag: 'chore', est: 'M', owner: 'EL', col: 'later' },
  { id: 'BIR-258', title: 'Slack integration: post on milestone close',        tag: 'feat',  est: 'M', owner: 'RS', col: 'later' },
  { id: 'BIR-221', title: 'Replace moment.js with date-fns across web',        tag: 'debt',  est: 'L', owner: 'TN', col: 'later' },
  { id: 'BIR-263', title: 'Drag-to-reorder columns on the board view',         tag: 'feat',  est: 'M', owner: 'JM', col: 'later' },
  { id: 'BIR-209', title: 'Upgrade Postgres minor + reindex search vectors',   tag: 'chore', est: 'M', owner: 'AK', col: 'later' },
  { id: 'BIR-251', title: 'Per-doc read receipts in the share dialog',         tag: 'feat',  est: 'L', owner: 'JM', col: 'later' },
  { id: 'BIR-240', title: 'Onboarding checklist resurfaces after dismissal',   tag: 'bug',   est: 'S', owner: 'TN', col: 'later' },
  { id: 'BIR-265', title: 'Add request-id to client error reports',            tag: 'debt',  est: 'S', owner: 'RS', col: 'later' },

  { id: 'BIR-198', title: 'Custom emoji reactions on comments',                tag: 'feat',  est: 'M', owner: 'EL', col: 'cut' },
  { id: 'BIR-204', title: 'Native Windows desktop wrapper',                    tag: 'feat',  est: 'L', owner: 'TN', col: 'cut' },
  { id: 'BIR-217', title: 'AI summary of long comment threads',                tag: 'feat',  est: 'L', owner: 'JM', col: 'cut' },
  { id: 'BIR-233', title: 'Public roadmap page with voting',                   tag: 'feat',  est: 'L', owner: 'RS', col: 'cut' }
];

var OWNER_CLASS = { AK: 'o1', JM: 'o2', RS: 'o3', TN: 'o4', EL: 'o1' };

var tickets = [];          // live state
var activeFilter = null;   // tag string or null
var dragId = null;

// ---------- build DOM ----------
var board = document.getElementById('board');
var colBodies = {};
var colCounts = {};
var colFoots = {};

COLUMNS.forEach(function (c) {
  var col = document.createElement('section');
  col.className = 'col';
  col.dataset.col = c.key;

  var head = document.createElement('div');
  head.className = 'col-head';
  var h2 = document.createElement('h2');
  h2.textContent = c.label;
  var count = document.createElement('span');
  count.className = 'count';
  head.appendChild(h2);
  head.appendChild(count);

  var body = document.createElement('div');
  body.className = 'col-body';

  var foot = document.createElement('div');
  foot.className = 'col-foot';

  col.appendChild(head);
  col.appendChild(body);
  col.appendChild(foot);
  board.appendChild(col);

  colBodies[c.key] = body;
  colCounts[c.key] = count;
  colFoots[c.key] = foot;

  // drop targets
  col.addEventListener('dragover', function (e) {
    e.preventDefault();
    e.dataTransfer.dropEffect = 'move';
    col.classList.add('dragover');
  });
  col.addEventListener('dragleave', function (e) {
    if (!col.contains(e.relatedTarget)) col.classList.remove('dragover');
  });
  col.addEventListener('drop', function (e) {
    e.preventDefault();
    col.classList.remove('dragover');
    if (!dragId) return;
    var t = tickets.find(function (x) { return x.id === dragId; });
    if (t && t.col !== c.key) { t.col = c.key; render(); }
  });
});

function makeCard(t) {
  var card = document.createElement('article');
  card.className = 'ticket';
  card.draggable = true;
  card.dataset.id = t.id;
  if (activeFilter && t.tag !== activeFilter) card.classList.add('dim');

  var top = document.createElement('div');
  top.className = 'ticket-top';
  var tid = document.createElement('span');
  tid.className = 'tid';
  tid.textContent = t.id;
  var tag = document.createElement('button');
  tag.type = 'button';
  tag.className = 'tag tag-' + t.tag;
  tag.textContent = t.tag;
  tag.addEventListener('click', function (e) {
    e.stopPropagation();
    activeFilter = (activeFilter === t.tag) ? null : t.tag;
    render();
  });
  var est = document.createElement('span');
  est.className = 'est';
  est.textContent = t.est;
  top.appendChild(tid);
  top.appendChild(tag);
  top.appendChild(est);

  var title = document.createElement('div');
  title.className = 'ttitle';
  title.textContent = t.title;

  var bot = document.createElement('div');
  bot.className = 'ticket-bot';
  var owner = document.createElement('span');
  owner.className = 'owner ' + (OWNER_CLASS[t.owner] || 'o1');
  owner.textContent = t.owner;
  bot.appendChild(owner);

  card.appendChild(top);
  card.appendChild(title);
  card.appendChild(bot);

  card.addEventListener('dragstart', function (e) {
    dragId = t.id;
    card.classList.add('dragging');
    e.dataTransfer.effectAllowed = 'move';
    e.dataTransfer.setData('text/plain', t.id);
  });
  card.addEventListener('dragend', function () {
    dragId = null;
    card.classList.remove('dragging');
    document.querySelectorAll('.col.dragover').forEach(function (el) {
      el.classList.remove('dragover');
    });
  });

  return card;
}

function render() {
  COLUMNS.forEach(function (c) { colBodies[c.key].innerHTML = ''; });
  var counts = {};
  var pts = {};
  COLUMNS.forEach(function (c) { counts[c.key] = 0; pts[c.key] = 0; });

  tickets.forEach(function (t) {
    colBodies[t.col].appendChild(makeCard(t));
    counts[t.col] += 1;
    pts[t.col] += EST_PTS[t.est] || 0;
  });

  COLUMNS.forEach(function (c) {
    colCounts[c.key].textContent = counts[c.key];
    colFoots[c.key].innerHTML = 'estimate <b>' + pts[c.key] + ' pt' + (pts[c.key] === 1 ? '' : 's') + '</b>';
  });

  var summary = document.getElementById('summary');
  summary.innerHTML = COLUMNS.map(function (c) {
    return '<b>' + counts[c.key] + '</b>&nbsp;' + c.key;
  }).join('<span class="dot">·</span>');

  var badge = document.getElementById('filterBadge');
  if (activeFilter) {
    badge.textContent = 'filter: ' + activeFilter + ' ×';
    badge.classList.add('on');
  } else {
    badge.classList.remove('on');
  }
}

document.getElementById('filterBadge').addEventListener('click', function () {
  activeFilter = null;
  render();
});

// ---------- export ----------
function buildMarkdown() {
  var lines = [];
  lines.push('# Acme — Cycle 14 triage');
  lines.push('');
  COLUMNS.forEach(function (c) {
    var rows = tickets.filter(function (t) { return t.col === c.key; });
    var pts = rows.reduce(function (s, t) { return s + (EST_PTS[t.est] || 0); }, 0);
    lines.push('## ' + c.label + ' (' + rows.length + ' · ' + pts + ' pts)');
    lines.push('');
    lines.push('_' + c.rationale + '_');
    lines.push('');
    rows.forEach(function (t) {
      lines.push('- **' + t.id + '** ' + t.title + ' — ' + t.tag + ', ' + t.est + ', ' + t.owner);
    });
    lines.push('');
  });
  return lines.join('\n');
}

var copyBtn = document.getElementById('copyBtn');
var copyTimer = null;
copyBtn.addEventListener('click', function () {
  var md = buildMarkdown();
  function flash() {
    copyBtn.textContent = 'Copied ✓';
    copyBtn.classList.add('copied');
    clearTimeout(copyTimer);
    copyTimer = setTimeout(function () {
      copyBtn.textContent = 'Copy as markdown';
      copyBtn.classList.remove('copied');
    }, 1200);
  }
  if (navigator.clipboard && navigator.clipboard.writeText) {
    navigator.clipboard.writeText(md).then(flash, flash);
  } else {
    var ta = document.createElement('textarea');
    ta.value = md;
    document.body.appendChild(ta);
    ta.select();
    try { document.execCommand('copy'); } catch (e) {}
    document.body.removeChild(ta);
    flash();
  }
});

document.getElementById('resetBtn').addEventListener('click', function () {
  tickets = INITIAL.map(function (t) { return Object.assign({}, t); });
  activeFilter = null;
  render();
});

// ---------- init ----------
tickets = INITIAL.map(function (t) { return Object.assign({}, t); });
render();
