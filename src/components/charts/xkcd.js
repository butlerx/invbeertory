export function bar(svg, options) {
  try {
    new chartXkcd.Bar(svg, options);
  } catch (e) {
    console.error(e);
  }
}

export function line(svg, options) {
  try {
    new chartXkcd.Line(svg, options);
  } catch (e) {
    console.error(e);
  }
}

export function pie(svg, options) {
  try {
    new chartXkcd.Pie(svg, options);
  } catch (e) {
    console.error(e);
  }
}

export function radar(svg, options) {
  try {
    new chartXkcd.Radar(svg, options);
  } catch (e) {
    console.error(e);
  }
}

export function stackedBar(svg, options) {
  try {
    new chartXkcd.StackedBar(svg, options);
  } catch (e) {
    console.error(e);
  }
}

export function xy(svg, options) {
  try {
    new chartXkcd.XY(svg, options);
  } catch (e) {
    console.error(e);
  }
}
