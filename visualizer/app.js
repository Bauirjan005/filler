// Filler Graphic Visualizer & Replay Analyzer
(function () {
  'use strict';

  // State
  let turns = [];
  let currentTurnIndex = 0;
  let isPlaying = false;
  let playInterval = null;
  let playbackSpeed = 200;
  let showHeatmap = false;
  let hoveredCell = null;

  // DOM Elements
  const canvas = document.getElementById('arenaCanvas');
  const ctx = canvas.getContext('2d');
  const viewport = document.getElementById('viewport');
  const hoverInfo = document.getElementById('hoverInfo');

  const turnDisplay = document.getElementById('turnDisplay');
  const gridDimDisplay = document.getElementById('gridDimDisplay');
  const p1ScoreEl = document.getElementById('p1Score');
  const p2ScoreEl = document.getElementById('p2Score');
  const p1BarEl = document.getElementById('p1Bar');
  const p2BarEl = document.getElementById('p2Bar');
  const p1PercentEl = document.getElementById('p1Percent');
  const p2PercentEl = document.getElementById('p2Percent');
  const p1NameEl = document.getElementById('p1Name');
  const p2NameEl = document.getElementById('p2Name');

  const pieceDimEl = document.getElementById('pieceDim');
  const piecePreviewEl = document.getElementById('piecePreview');

  const playBtn = document.getElementById('playBtn');
  const prevBtn = document.getElementById('prevBtn');
  const nextBtn = document.getElementById('nextBtn');
  const startBtn = document.getElementById('startBtn');
  const endBtn = document.getElementById('endBtn');
  const timelineSlider = document.getElementById('timelineSlider');
  const speedSelect = document.getElementById('speedSelect');
  const heatmapToggle = document.getElementById('heatmapToggle');

  const demoBtn = document.getElementById('demoBtn');
  const importBtn = document.getElementById('importBtn');
  const fileInput = document.getElementById('fileInput');
  const importModal = document.getElementById('importModal');
  const closeModalBtn = document.getElementById('closeModalBtn');
  const cancelModalBtn = document.getElementById('cancelModalBtn');
  const parseLogBtn = document.getElementById('parseLogBtn');
  const logTextarea = document.getElementById('logTextarea');

  // Colors
  const COLORS = {
    empty: '#111420',
    gridLine: 'rgba(255, 255, 255, 0.04)',
    p1: '#00f0ff',
    p1Last: '#b2fbff',
    p1Glow: 'rgba(0, 240, 255, 0.5)',
    p2: '#ff007f',
    p2Last: '#ffb2d9',
    p2Glow: 'rgba(255, 0, 127, 0.5)',
  };

  // Setup Event Listeners
  function init() {
    setupControls();
    setupCanvasEvents();
    window.addEventListener('resize', renderCurrentTurn);
    generateDemoBattle();
  }

  function setupControls() {
    playBtn.addEventListener('click', togglePlay);
    prevBtn.addEventListener('click', () => step(-1));
    nextBtn.addEventListener('click', () => step(1));
    startBtn.addEventListener('click', () => goToTurn(0));
    endBtn.addEventListener('click', () => goToTurn(turns.length - 1));

    timelineSlider.addEventListener('input', (e) => {
      goToTurn(parseInt(e.target.value, 10));
    });

    speedSelect.addEventListener('change', (e) => {
      playbackSpeed = parseInt(e.target.value, 10);
      if (isPlaying) {
        clearInterval(playInterval);
        playInterval = setInterval(autoPlayStep, playbackSpeed);
      }
    });

    heatmapToggle.addEventListener('change', (e) => {
      showHeatmap = e.target.checked;
      renderCurrentTurn();
    });

    demoBtn.addEventListener('click', generateDemoBattle);

    importBtn.addEventListener('click', () => {
      importModal.style.display = 'flex';
    });

    closeModalBtn.addEventListener('click', () => (importModal.style.display = 'none'));
    cancelModalBtn.addEventListener('click', () => (importModal.style.display = 'none'));

    parseLogBtn.addEventListener('click', () => {
      const text = logTextarea.value.trim();
      if (text) {
        parseEngineLog(text);
        importModal.style.display = 'none';
      }
    });
  }

  function setupCanvasEvents() {
    canvas.addEventListener('mousemove', (e) => {
      if (!turns.length) return;
      const rect = canvas.getBoundingClientRect();
      const current = turns[currentTurnIndex];
      const cellSize = canvas.width / current.width;

      const mouseX = e.clientX - rect.left;
      const mouseY = e.clientY - rect.top;

      const gx = Math.floor(mouseX / cellSize);
      const gy = Math.floor(mouseY / cellSize);

      if (gx >= 0 && gx < current.width && gy >= 0 && gy < current.height) {
        hoveredCell = { x: gx, y: gy };
        const cellChar = current.grid[gy][gx];
        let owner = 'Empty';
        if (cellChar === '@' || cellChar === 'a') owner = 'Player 1 (@)';
        else if (cellChar === '$' || cellChar === 's') owner = 'Player 2 ($)';

        hoverInfo.style.display = 'block';
        hoverInfo.style.left = `${e.clientX - rect.left + 15}px`;
        hoverInfo.style.top = `${e.clientY - rect.top + 15}px`;
        hoverInfo.innerHTML = `<strong>(${gx}, ${gy})</strong> &nbsp; ${owner}`;
      } else {
        hoveredCell = null;
        hoverInfo.style.display = 'none';
      }
    });

    canvas.addEventListener('mouseleave', () => {
      hoveredCell = null;
      hoverInfo.style.display = 'none';
    });
  }

  // Playback Logic
  function togglePlay() {
    isPlaying = !isPlaying;
    if (isPlaying) {
      playBtn.textContent = '⏸';
      playBtn.style.background = '#ff007f';
      playInterval = setInterval(autoPlayStep, playbackSpeed);
    } else {
      pausePlayback();
    }
  }

  function pausePlayback() {
    isPlaying = false;
    playBtn.textContent = '▶';
    playBtn.style.background = '#00f0ff';
    clearInterval(playInterval);
  }

  function autoPlayStep() {
    if (currentTurnIndex < turns.length - 1) {
      goToTurn(currentTurnIndex + 1);
    } else {
      pausePlayback();
    }
  }

  function step(dir) {
    pausePlayback();
    const target = currentTurnIndex + dir;
    if (target >= 0 && target < turns.length) {
      goToTurn(target);
    }
  }

  function goToTurn(index) {
    if (index >= 0 && index < turns.length) {
      currentTurnIndex = index;
      timelineSlider.value = index;
      renderCurrentTurn();
    }
  }

  // Rendering
  function renderCurrentTurn() {
    if (!turns.length) return;
    const turn = turns[currentTurnIndex];

    turnDisplay.textContent = `${currentTurnIndex + 1} / ${turns.length}`;
    gridDimDisplay.textContent = `${turn.width} × ${turn.height}`;

    p1ScoreEl.textContent = turn.p1Score;
    p2ScoreEl.textContent = turn.p2Score;

    const total = turn.p1Score + turn.p2Score || 1;
    const p1Pct = ((turn.p1Score / total) * 100).toFixed(1);
    const p2Pct = ((turn.p2Score / total) * 100).toFixed(1);

    p1PercentEl.textContent = `${p1Pct}%`;
    p2PercentEl.textContent = `${p2Pct}%`;
    p1BarEl.style.width = `${p1Pct}%`;
    p2BarEl.style.width = `${p2Pct}%`;

    // Render piece preview
    renderPiecePreview(turn.piece);

    // Canvas sizing
    const vpRect = viewport.getBoundingClientRect();
    const maxW = vpRect.width - 24;
    const maxH = vpRect.height - 24;

    const aspect = turn.width / turn.height;
    let cw = maxW;
    let ch = cw / aspect;
    if (ch > maxH) {
      ch = maxH;
      cw = ch * aspect;
    }

    canvas.width = cw;
    canvas.height = ch;

    const cellSize = cw / turn.width;

    // Draw background
    ctx.fillStyle = COLORS.empty;
    ctx.fillRect(0, 0, cw, ch);

    // Draw Heatmap if enabled
    let heatmap = null;
    if (showHeatmap) {
      heatmap = computeBFSHeatmap(turn.grid, turn.width, turn.height);
    }

    // Draw grid cells
    for (let y = 0; y < turn.height; y++) {
      for (let x = 0; x < turn.width; x++) {
        const char = turn.grid[y][x];
        const px = x * cellSize;
        const py = y * cellSize;
        const pad = Math.max(1, cellSize * 0.08);
        const w = cellSize - pad * 2;
        const h = cellSize - pad * 2;

        if (showHeatmap && heatmap && heatmap[y][x] !== Infinity && char === '.') {
          const d = heatmap[y][x];
          const heatRatio = Math.max(0, 1 - d / (turn.width / 1.5));
          ctx.fillStyle = `rgba(112, 0, 255, ${heatRatio * 0.4})`;
          ctx.fillRect(px, py, cellSize, cellSize);
        }

        if (char === '@') {
          ctx.fillStyle = COLORS.p1;
          drawRoundedRect(ctx, px + pad, py + pad, w, h, 2);
        } else if (char === 'a') {
          ctx.fillStyle = COLORS.p1Last;
          ctx.shadowColor = COLORS.p1Glow;
          ctx.shadowBlur = 8;
          drawRoundedRect(ctx, px + pad, py + pad, w, h, 2);
          ctx.shadowBlur = 0;
        } else if (char === '$') {
          ctx.fillStyle = COLORS.p2;
          drawRoundedRect(ctx, px + pad, py + pad, w, h, 2);
        } else if (char === 's') {
          ctx.fillStyle = COLORS.p2Last;
          ctx.shadowColor = COLORS.p2Glow;
          ctx.shadowBlur = 8;
          drawRoundedRect(ctx, px + pad, py + pad, w, h, 2);
          ctx.shadowBlur = 0;
        } else {
          // Empty grid lines
          ctx.strokeStyle = COLORS.gridLine;
          ctx.strokeRect(px, py, cellSize, cellSize);
        }
      }
    }
  }

  function drawRoundedRect(ctx, x, y, width, height, radius) {
    ctx.beginPath();
    ctx.moveTo(x + radius, y);
    ctx.lineTo(x + width - radius, y);
    ctx.quadraticCurveTo(x + width, y, x + width, y + radius);
    ctx.lineTo(x + width, y + height - radius);
    ctx.quadraticCurveTo(x + width, y + height, x + width - radius, y + height);
    ctx.lineTo(x + radius, y + height);
    ctx.quadraticCurveTo(x, y + height, x, y + height - radius);
    ctx.lineTo(x, y + radius);
    ctx.quadraticCurveTo(x, y, x + radius, y);
    ctx.closePath();
    ctx.fill();
  }

  function computeBFSHeatmap(grid, width, height) {
    const dist = Array.from({ length: height }, () => Array(width).fill(Infinity));
    const queue = [];

    for (let y = 0; y < height; y++) {
      for (let x = 0; x < width; x++) {
        if (grid[y][x] === '$' || grid[y][x] === 's') {
          dist[y][x] = 0;
          queue.push([x, y]);
        }
      }
    }

    let head = 0;
    const dirs = [[0, 1], [0, -1], [1, 0], [-1, 0]];
    while (head < queue.length) {
      const [cx, cy] = queue[head++];
      const d = dist[cy][cx];
      for (const [dx, dy] of dirs) {
        const nx = cx + dx;
        const ny = cy + dy;
        if (nx >= 0 && nx < width && ny >= 0 && ny < height) {
          if (dist[ny][nx] > d + 1) {
            dist[ny][nx] = d + 1;
            queue.push([nx, ny]);
          }
        }
      }
    }
    return dist;
  }

  function renderPiecePreview(piece) {
    if (!piece) {
      pieceDimEl.textContent = 'None';
      piecePreviewEl.innerHTML = '';
      return;
    }
    pieceDimEl.textContent = `${piece.width} × ${piece.height}`;
    let html = `<div class="piece-grid" style="grid-template-columns: repeat(${piece.width}, 14px);">`;
    for (const row of piece.raw) {
      for (const ch of row) {
        const isSolid = ch !== '.';
        html += `<div class="piece-cell ${isSolid ? 'solid' : ''}"></div>`;
      }
    }
    html += '</div>';
    piecePreviewEl.innerHTML = html;
  }

  // Demo Battle Generator
  function generateDemoBattle() {
    pausePlayback();
    const width = 20;
    const height = 15;
    turns = [];

    let grid = Array.from({ length: height }, () => Array(width).fill('.'));
    grid[2][9] = '@';
    grid[12][9] = '$';

    // Simulate 35 turns of realistic battle
    const piecesPool = [
      { w: 4, h: 1, raw: ['.OO.'] },
      { w: 3, h: 2, raw: ['##.', '.##'] },
      { w: 2, h: 3, raw: ['#.', '##', '.#'] },
      { w: 3, h: 3, raw: ['.#.', '###', '.#.'] },
      { w: 2, h: 2, raw: ['##', '##'] },
    ];

    let p1Cells = [[9, 2]];
    let p2Cells = [[9, 12]];

    for (let t = 0; t < 40; t++) {
      const p = piecesPool[t % piecesPool.length];

      // P1 move towards P2
      if (p1Cells.length > 0) {
        const lastP1 = p1Cells[p1Cells.length - 1];
        const nextX = Math.min(width - 2, Math.max(1, lastP1[0] + (t % 3 === 0 ? 1 : 0)));
        const nextY = Math.min(height - 2, lastP1[1] + 1);
        grid[nextY][nextX] = 'a';
        p1Cells.push([nextX, nextY]);
      }

      // P2 move towards center
      if (p2Cells.length > 0 && t < 22) {
        const lastP2 = p2Cells[p2Cells.length - 1];
        const nextX = Math.max(1, lastP2[0] - (t % 2 === 0 ? 1 : 0));
        const nextY = Math.max(1, lastP2[1] - 1);
        if (grid[nextY][nextX] === '.') {
          grid[nextY][nextX] = 's';
          p2Cells.push([nextX, nextY]);
        }
      }

      // Calculate score
      let p1Score = 0, p2Score = 0;
      const copyGrid = grid.map((r, y) => r.map((c, x) => {
        if (c === '@' || c === 'a') p1Score++;
        if (c === '$' || c === 's') p2Score++;
        return c;
      }));

      turns.push({
        width,
        height,
        grid: copyGrid,
        piece: p,
        p1Score,
        p2Score,
      });

      // Convert 'a' and 's' to permanent '@' and '$'
      for (let y = 0; y < height; y++) {
        for (let x = 0; x < width; x++) {
          if (grid[y][x] === 'a') grid[y][x] = '@';
          if (grid[y][x] === 's') grid[y][x] = '$';
        }
      }
    }

    timelineSlider.max = turns.length - 1;
    timelineSlider.value = 0;
    currentTurnIndex = 0;
    renderCurrentTurn();
  }

  // Parse Real Engine Log
  function parseEngineLog(logText) {
    pausePlayback();
    const lines = logText.split('\n');
    const parsedTurns = [];

    let currentGrid = null;
    let currentPiece = null;
    let bWidth = 0, bHeight = 0;
    let p1Name = 'Player 1', p2Name = 'Player 2';

    let i = 0;
    while (i < lines.length) {
      const line = lines[i].trim();

      if (line.includes('$$$ exec p1')) {
        const match = line.match(/\[(.*?)\]/);
        if (match) p1Name = match[1];
      } else if (line.includes('$$$ exec p2')) {
        const match = line.match(/\[(.*?)\]/);
        if (match) p2Name = match[1];
      } else if (line.toLowerCase().startsWith('anfield') || line.toLowerCase().startsWith('plateau')) {
        const parts = line.split(/\s+/);
        if (parts.length >= 3) {
          bWidth = parseInt(parts[1], 10);
          bHeight = parseInt(parts[2].replace(':', ''), 10);

          // Skip column index line
          i++;
          const gridRows = [];
          for (let r = 0; r < bHeight && i + 1 < lines.length; r++) {
            i++;
            const rowStr = lines[i].replace(/^\s*\d+\s*/, '').trim();
            gridRows.push(rowStr.split('').slice(0, bWidth));
          }
          currentGrid = gridRows;
        }
      } else if (line.toLowerCase().startsWith('piece')) {
        const parts = line.split(/\s+/);
        if (parts.length >= 3) {
          const pw = parseInt(parts[1], 10);
          const ph = parseInt(parts[2].replace(':', ''), 10);
          const pieceRows = [];
          for (let pr = 0; pr < ph && i + 1 < lines.length; pr++) {
            i++;
            pieceRows.push(lines[i].trim());
          }
          currentPiece = { width: pw, height: ph, raw: pieceRows };

          if (currentGrid) {
            let p1Score = 0, p2Score = 0;
            for (let r = 0; r < bHeight; r++) {
              for (let c = 0; c < bWidth; c++) {
                const ch = currentGrid[r] ? currentGrid[r][c] : '.';
                if (ch === '@' || ch === 'a') p1Score++;
                if (ch === '$' || ch === 's') p2Score++;
              }
            }

            parsedTurns.push({
              width: bWidth,
              height: bHeight,
              grid: currentGrid,
              piece: currentPiece,
              p1Score,
              p2Score,
            });
          }
        }
      }
      i++;
    }

    if (parsedTurns.length > 0) {
      turns = parsedTurns;
      p1NameEl.textContent = p1Name;
      p2NameEl.textContent = p2Name;
      timelineSlider.max = turns.length - 1;
      timelineSlider.value = 0;
      currentTurnIndex = 0;
      renderCurrentTurn();
    } else {
      alert('Could not find Anfield turns in the provided log. Please check the log format.');
    }
  }

  window.addEventListener('DOMContentLoaded', init);
})();
