import { ref, onUnmounted, computed } from 'vue';

export interface ResizeState {
  isResizing: boolean;
  resizingPanel: 'left' | 'right' | 'top' | 'bottom' | null;
  startX: number;
  startY: number;
  startSize: number;
}

export interface PanelSizes {
  leftPanelWidth: number;
  rightPanelWidth: number;
  topPanelHeight: number;
  bottomPanelHeight: number;
}

export interface ResizeConstraints {
  MIN_PANEL_WIDTH: number;
  MAX_PANEL_WIDTH: number;
  MIN_PANEL_HEIGHT: number;
  MAX_PANEL_HEIGHT: number;
}

export function useResizePanel(
  initialSizes: PanelSizes,
  constraints: ResizeConstraints
) {
  // Panel dimensions
  const leftPanelWidth = ref(initialSizes.leftPanelWidth);
  const rightPanelWidth = ref(initialSizes.rightPanelWidth);
  const topPanelHeight = ref(initialSizes.topPanelHeight);
  const bottomPanelHeight = ref(initialSizes.bottomPanelHeight);

  // Resize state
  const state = ref<ResizeState>({
    isResizing: false,
    resizingPanel: null,
    startX: 0,
    startY: 0,
    startSize: 0,
  });

  const startResize = (panel: ResizeState['resizingPanel'], event: MouseEvent) => {
    if (!panel) return;

    state.value = {
      isResizing: true,
      resizingPanel: panel,
      startX: event.pageX,
      startY: event.pageY,
      startSize: getPanelSize(panel),
    };

    document.addEventListener('mousemove', handleMouseMove);
    document.addEventListener('mouseup', stopResize);
    document.body.style.userSelect = 'none';
  };

  const getPanelSize = (panel: NonNullable<ResizeState['resizingPanel']>) => {
    switch (panel) {
      case 'left': return leftPanelWidth.value;
      case 'right': return rightPanelWidth.value;
      case 'top': return topPanelHeight.value;
      case 'bottom': return bottomPanelHeight.value;
    }
  };

  const handleMouseMove = (event: MouseEvent) => {
    if (!state.value.isResizing || !state.value.resizingPanel) return;

    const { resizingPanel, startX, startY, startSize } = state.value;

    if (resizingPanel === 'left' || resizingPanel === 'right') {
      const diff = event.pageX - startX;
      const newWidth = resizingPanel === 'left'
        ? startSize + diff
        : startSize - diff;

      const constrainedWidth = Math.max(
        constraints.MIN_PANEL_WIDTH,
        Math.min(constraints.MAX_PANEL_WIDTH, newWidth)
      );

      if (resizingPanel === 'left') {
        leftPanelWidth.value = constrainedWidth;
      } else {
        rightPanelWidth.value = constrainedWidth;
      }
    } else {
      const diff = event.pageY - startY;
      const newHeight = resizingPanel === 'top'
        ? startSize + diff
        : startSize - diff;

      const constrainedHeight = Math.max(
        constraints.MIN_PANEL_HEIGHT,
        Math.min(constraints.MAX_PANEL_HEIGHT, newHeight)
      );

      if (resizingPanel === 'top') {
        topPanelHeight.value = constrainedHeight;
      } else {
        bottomPanelHeight.value = constrainedHeight;
      }
    }
  };

  const stopResize = () => {
    state.value.isResizing = false;
    state.value.resizingPanel = null;
    document.removeEventListener('mousemove', handleMouseMove);
    document.removeEventListener('mouseup', stopResize);
    document.body.style.userSelect = '';
  };

  onUnmounted(() => {
    document.removeEventListener('mousemove', handleMouseMove);
    document.removeEventListener('mouseup', stopResize);
  });

  return {
    leftPanelWidth,
    rightPanelWidth,
    topPanelHeight,
    bottomPanelHeight,
    startResize,
    isResizing: computed(() => state.value.isResizing),
    resizingPanel: computed(() => state.value.resizingPanel),
  };
} 