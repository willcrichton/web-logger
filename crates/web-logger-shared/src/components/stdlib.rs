use std::{collections::HashMap, rc::Rc};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use type_uuid::TypeUuid;
use yew::{self, function_component, html, virtual_dom::VComp, Properties};
use yew_chart::{
  axis::{Axis, Orientation, Scale},
  linear_axis_scale::LinearScale,
  series::{Series, Type},
};

use crate::into_component;

thread_local! {
  pub(super) static COMPONENTS: HashMap<type_uuid::Bytes, fn(&Value) -> VComp> = maplit::hashmap! {
    <Circle as TypeUuid>::UUID => make_circle as fn(&Value) -> VComp,
    <Chart as TypeUuid>::UUID => make_chart as fn(&Value) -> VComp
  }
}

#[derive(Serialize, Deserialize, PartialEq, Default, TypeUuid)]
#[uuid = "d4adfc76-f5f4-40b0-8e28-8a51a12f5e46"]
pub struct Circle {
  pub radius: f64,
}

#[derive(Properties, PartialEq, Default)]
pub struct CircleProps {
  circle: Circle,
}

impl CircleProps {
  pub fn new(circle: Circle) -> Self {
    CircleProps { circle }
  }
}

#[function_component(CircleView)]
pub fn circle_view(props: &CircleProps) -> Html {
  let &Circle { radius } = &props.circle;
  html! {
    <svg class="circle" viewBox={"0 0 2 2"} style={format!("width: {}px", radius)}>
      <circle cx={ 1 } cy={ 1 } r={ 1 } fill={"red"} />
    </svg>
  }
}

into_component!(Circle, CircleProps, CircleView, make_circle);

#[derive(Serialize, Deserialize, PartialEq, Default, TypeUuid)]
#[uuid = "7a46734a-490a-40a2-a92b-c3d44c36c336"]
pub struct Chart {
  pub data: Vec<(f32, f32)>,
}

#[derive(Properties, PartialEq, Default)]
pub struct ChartProps {
  chart: Chart,
}

impl ChartProps {
  pub fn new(chart: Chart) -> Self {
    ChartProps { chart }
  }
}

const WIDTH: f32 = 500.0;
const HEIGHT: f32 = 300.0;
const MARGIN: f32 = 50.0;
const TICK_LENGTH: f32 = 10.0;

#[function_component(ChartView)]
pub fn chart_view(props: &ChartProps) -> Html {
  let Chart { data } = &props.chart;
  let (x, y): (Vec<_>, Vec<_>) = data.iter().copied().unzip();
  let range = |v: Vec<f32>| {
    v.clone().into_iter().reduce(|a, b| a.min(b)).unwrap()
      .. v.clone().into_iter().reduce(|a, b| a.max(b)).unwrap()
  };
  let h_scale = Rc::new(LinearScale::new(range(x), 1.0)) as Rc<dyn Scale>;
  let v_scale = Rc::new(LinearScale::new(range(y), 1.0)) as Rc<dyn Scale>;
  let data = Rc::new(
    data
      .clone()
      .into_iter()
      .map(|(x, y)| (x, y, None))
      .collect::<Vec<_>>(),
  );

  html! {
    <svg class="chart" viewBox={format!("0 0 {} {}", WIDTH, HEIGHT)} preserveAspectRatio="none">
      <Series
          series_type={Type::Line}
          name={"some-series"}
          data={data}
          horizontal_scale={Rc::clone(&h_scale)}
          horizontal_scale_step={1.}
          vertical_scale={Rc::clone(&v_scale)}
          x={MARGIN} y={MARGIN} width={WIDTH - (MARGIN * 2.0)} height={HEIGHT - (MARGIN * 2.0)} />

      <Axis
          name="some-y-axis"
          orientation={Orientation::Left}
          scale={Rc::clone(&v_scale)}
          x1={MARGIN} y1={MARGIN} xy2={HEIGHT - MARGIN}
          tick_len={TICK_LENGTH}
          title={"".to_string()} />

      <Axis
          name="some-x-axis"
          orientation={Orientation::Bottom}
          scale={Rc::clone(&h_scale)}
          x1={MARGIN} y1={HEIGHT - MARGIN} xy2={WIDTH - MARGIN}
          tick_len={TICK_LENGTH}
          title={"".to_string()} />
    </svg>
  }
}

into_component!(Chart, ChartProps, ChartView, make_chart);
