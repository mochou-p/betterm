// mochou-p/betterm/src/color.rs

/// get [`AnsiColor`]
pub mod ansi {
    use super::AnsiColor;

    /// ANSI black   (0)
    pub fn   black() -> AnsiColor { AnsiColor { number: 0, bright: false } }
    /// ANSI red     (1)
    pub fn     red() -> AnsiColor { AnsiColor { number: 1, bright: false } }
    /// ANSI green   (2)
    pub fn   green() -> AnsiColor { AnsiColor { number: 2, bright: false } }
    /// ANSI yellow  (3)
    pub fn  yellow() -> AnsiColor { AnsiColor { number: 3, bright: false } }
    /// ANSI blue    (4)
    pub fn    blue() -> AnsiColor { AnsiColor { number: 4, bright: false } }
    /// ANSI magenta (5)
    pub fn magenta() -> AnsiColor { AnsiColor { number: 5, bright: false } }
    /// ANSI cyan    (6)
    pub fn    cyan() -> AnsiColor { AnsiColor { number: 6, bright: false } }
    /// ANSI white   (7)
    pub fn   white() -> AnsiColor { AnsiColor { number: 7, bright: false } }
}

/// these are the colors of your terminal's theme,
/// everyone will see different colors depending on their own terminal configuration
#[derive(Clone, Copy, Debug)]
pub struct AnsiColor {
        number: u8,
    /// whether this is a normal or a bright version of a color
    pub bright: bool
}

impl AnsiColor {
    /// make this the bright version of a color
    pub fn bright(self) -> Self { Self { number: self.number, bright: true  } }
    /// make this the normal version of a color
    pub fn normal(self) -> Self { Self { number: self.number, bright: false } }
}

/// get an [`LutColor`]
pub fn lut(index: u8) -> LutColor { LutColor { index } }

/// 8-bit lookup table (256 palette)
/// 
/// <hr>
/// standard colors (0..=7)
/// <div style="display: flex;">
///     <div style="flex: 1; display: grid; place-items: center; background: #000000; color: #fff;">0</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #800000; color: #fff;">1</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #008000; color: #fff;">2</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #808000; color: #fff;">3</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #000080; color: #fff;">4</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #800080; color: #fff;">5</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #008080; color: #fff;">6</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #c0c0c0; color: #fff;">7</div>
/// </div>
/// <br>
/// high-intensity colors (8..=15)
/// <div style="display: flex;">
///     <div style="flex: 1; display: grid; place-items: center; background: #808080; color: #000;">8</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #ff0000; color: #000;">9</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #00ff00; color: #000;">10</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #ffff00; color: #000;">11</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #0000ff; color: #000;">12</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #ff00ff; color: #000;">13</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #00ffff; color: #000;">14</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #ffffff; color: #000;">15</div>
/// </div>
/// <br>
/// 216 "web-safe" colors (16..=231)
/// <div style="display: flex;">
///     <div style="flex: 1; display: grid; place-items: center; background: #000000; color: #fff;">16</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #00005f; color: #fff;">17</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #000087; color: #fff;">18</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #0000af; color: #fff;">19</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #0000d7; color: #fff;">20</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #0000ff; color: #fff;">21</div>
/// </div>
/// <div style="display: flex;">
///     <div style="flex: 1; display: grid; place-items: center; background: #005f00; color: #fff;">22</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #005f5f; color: #fff;">23</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #005f87; color: #fff;">24</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #005faf; color: #fff;">25</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #005fd7; color: #fff;">26</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #005fff; color: #fff;">27</div>
/// </div>
/// <div style="display: flex;">
///     <div style="flex: 1; display: grid; place-items: center; background: #008700; color: #fff;">28</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #00875f; color: #fff;">29</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #008787; color: #fff;">30</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #0087af; color: #fff;">31</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #0087d7; color: #fff;">32</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #0087ff; color: #fff;">33</div>
/// </div>
/// <div style="display: flex;">
///     <div style="flex: 1; display: grid; place-items: center; background: #00af00; color: #000;">34</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #00af5f; color: #000;">35</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #00af87; color: #000;">36</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #00afaf; color: #000;">37</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #00afd7; color: #000;">38</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #00afff; color: #000;">39</div>
/// </div>
/// <div style="display: flex;">
///     <div style="flex: 1; display: grid; place-items: center; background: #00d700; color: #000;">40</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #00d75f; color: #000;">41</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #00d787; color: #000;">42</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #00d7af; color: #000;">43</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #00d7d7; color: #000;">44</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #00d7ff; color: #000;">45</div>
/// </div>
/// <div style="display: flex;">
///     <div style="flex: 1; display: grid; place-items: center; background: #00ff00; color: #000;">46</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #00ff5f; color: #000;">47</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #00ff87; color: #000;">48</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #00ffaf; color: #000;">49</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #00ffd7; color: #000;">50</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #00ffff; color: #000;">51</div>
/// </div>
/// <div style="display: flex;">
///     <div style="flex: 1; display: grid; place-items: center; background: #5f0000; color: #fff;">52</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #5f005f; color: #fff;">53</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #5f0087; color: #fff;">54</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #5f00af; color: #fff;">55</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #5f00d7; color: #fff;">56</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #5f00ff; color: #fff;">57</div>
/// </div>
/// <div style="display: flex;">
///     <div style="flex: 1; display: grid; place-items: center; background: #5f5f00; color: #fff;">58</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #5f5f5f; color: #fff;">59</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #5f5f87; color: #fff;">60</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #5f5faf; color: #fff;">61</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #5f5fd7; color: #fff;">62</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #5f5fff; color: #fff;">63</div>
/// </div>
/// <div style="display: flex;">
///     <div style="flex: 1; display: grid; place-items: center; background: #5f8700; color: #fff;">64</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #5f875f; color: #fff;">65</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #5f8787; color: #fff;">66</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #5f87af; color: #fff;">67</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #5f87d7; color: #fff;">68</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #5f87ff; color: #fff;">69</div>
/// </div>
/// <div style="display: flex;">
///     <div style="flex: 1; display: grid; place-items: center; background: #5faf00; color: #000;">70</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #5faf5f; color: #000;">71</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #5faf87; color: #000;">72</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #5fafaf; color: #000;">73</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #5fafd7; color: #000;">74</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #5fafff; color: #000;">75</div>
/// </div>
/// <div style="display: flex;">
///     <div style="flex: 1; display: grid; place-items: center; background: #5fd700; color: #000;">76</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #5fd75f; color: #000;">77</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #5fd787; color: #000;">78</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #5fd7af; color: #000;">79</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #5fd7d7; color: #000;">80</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #5fd7ff; color: #000;">81</div>
/// </div>
/// <div style="display: flex;">
///     <div style="flex: 1; display: grid; place-items: center; background: #5fff00; color: #000;">82</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #5fff5f; color: #000;">83</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #5fff87; color: #000;">84</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #5fffaf; color: #000;">85</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #5fffd7; color: #000;">86</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #5fffff; color: #000;">87</div>
/// </div>
/// <div style="display: flex;">
///     <div style="flex: 1; display: grid; place-items: center; background: #870000; color: #fff;">88</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #87005f; color: #fff;">89</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #870087; color: #fff;">90</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #8700af; color: #fff;">91</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #8700d7; color: #fff;">92</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #8700ff; color: #fff;">93</div>
/// </div>
/// <div style="display: flex;">
///     <div style="flex: 1; display: grid; place-items: center; background: #875f00; color: #fff;">94</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #875f5f; color: #fff;">95</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #875f87; color: #fff;">96</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #875faf; color: #fff;">97</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #875fd7; color: #fff;">98</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #875fff; color: #fff;">99</div>
/// </div>
/// <div style="display: flex;">
///     <div style="flex: 1; display: grid; place-items: center; background: #878700; color: #fff;">100</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #87875f; color: #fff;">101</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #878787; color: #fff;">102</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #8787af; color: #fff;">103</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #8787d7; color: #fff;">104</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #8787ff; color: #fff;">105</div>
/// </div>
/// <div style="display: flex;">
///     <div style="flex: 1; display: grid; place-items: center; background: #87af00; color: #000;">106</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #87af5f; color: #000;">107</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #87af87; color: #000;">108</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #87afaf; color: #000;">109</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #87afd7; color: #000;">110</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #87afff; color: #000;">111</div>
/// </div>
/// <div style="display: flex;">
///     <div style="flex: 1; display: grid; place-items: center; background: #87d700; color: #000;">112</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #87d75f; color: #000;">113</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #87d787; color: #000;">114</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #87d7af; color: #000;">115</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #87d7d7; color: #000;">116</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #87d7ff; color: #000;">117</div>
/// </div>
/// <div style="display: flex;">
///     <div style="flex: 1; display: grid; place-items: center; background: #87ff00; color: #000;">118</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #87ff5f; color: #000;">119</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #87ff87; color: #000;">120</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #87ffaf; color: #000;">121</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #87ffd7; color: #000;">122</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #87ffff; color: #000;">123</div>
/// </div>
/// <div style="display: flex;">
///     <div style="flex: 1; display: grid; place-items: center; background: #af0000; color: #fff;">124</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #af005f; color: #fff;">125</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #af0087; color: #fff;">126</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #af00af; color: #fff;">127</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #af00d7; color: #fff;">128</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #af00ff; color: #fff;">129</div>
/// </div>
/// <div style="display: flex;">
///     <div style="flex: 1; display: grid; place-items: center; background: #af5f00; color: #fff;">130</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #af5f5f; color: #fff;">131</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #af5f87; color: #fff;">132</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #af5faf; color: #fff;">133</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #af5fd7; color: #fff;">134</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #af5fff; color: #fff;">135</div>
/// </div>
/// <div style="display: flex;">
///     <div style="flex: 1; display: grid; place-items: center; background: #af8700; color: #fff;">136</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #af875f; color: #fff;">137</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #af8787; color: #fff;">138</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #af87af; color: #fff;">139</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #af87d7; color: #fff;">140</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #af87ff; color: #fff;">141</div>
/// </div>
/// <div style="display: flex;">
///     <div style="flex: 1; display: grid; place-items: center; background: #afaf00; color: #000;">142</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #afaf5f; color: #000;">143</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #afaf87; color: #000;">144</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #afafaf; color: #000;">145</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #afafd7; color: #000;">146</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #afafff; color: #000;">147</div>
/// </div>
/// <div style="display: flex;">
///     <div style="flex: 1; display: grid; place-items: center; background: #afd700; color: #000;">148</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #afd75f; color: #000;">149</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #afd787; color: #000;">150</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #afd7af; color: #000;">151</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #afd7d7; color: #000;">152</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #afd7ff; color: #000;">153</div>
/// </div>
/// <div style="display: flex;">
///     <div style="flex: 1; display: grid; place-items: center; background: #afff00; color: #000;">154</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #afff5f; color: #000;">155</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #afff87; color: #000;">156</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #afffaf; color: #000;">157</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #afffd7; color: #000;">158</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #afffff; color: #000;">159</div>
/// </div>
/// <div style="display: flex;">
///     <div style="flex: 1; display: grid; place-items: center; background: #d70000; color: #fff;">160</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #d7005f; color: #fff;">161</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #d70087; color: #fff;">162</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #d700af; color: #fff;">163</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #d700d7; color: #fff;">164</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #d700ff; color: #fff;">165</div>
/// </div>
/// <div style="display: flex;">
///     <div style="flex: 1; display: grid; place-items: center; background: #d75f00; color: #fff;">166</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #d75f5f; color: #fff;">167</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #d75f87; color: #fff;">168</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #d75faf; color: #fff;">169</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #d75fd7; color: #fff;">170</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #d75fff; color: #fff;">171</div>
/// </div>
/// <div style="display: flex;">
///     <div style="flex: 1; display: grid; place-items: center; background: #d78700; color: #fff;">172</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #d7875f; color: #fff;">173</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #d78787; color: #fff;">174</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #d787af; color: #fff;">175</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #d787d7; color: #fff;">176</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #d787ff; color: #fff;">177</div>
/// </div>
/// <div style="display: flex;">
///     <div style="flex: 1; display: grid; place-items: center; background: #d7af00; color: #000;">178</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #d7af5f; color: #000;">179</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #d7af87; color: #000;">180</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #d7afaf; color: #000;">181</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #d7afd7; color: #000;">182</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #d7afff; color: #000;">183</div>
/// </div>
/// <div style="display: flex;">
///     <div style="flex: 1; display: grid; place-items: center; background: #d7d700; color: #000;">184</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #d7d75f; color: #000;">185</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #d7d787; color: #000;">186</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #d7d7af; color: #000;">187</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #d7d7d7; color: #000;">188</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #d7d7ff; color: #000;">189</div>
/// </div>
/// <div style="display: flex;">
///     <div style="flex: 1; display: grid; place-items: center; background: #d7ff00; color: #000;">190</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #d7ff5f; color: #000;">191</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #d7ff87; color: #000;">192</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #d7ffaf; color: #000;">193</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #d7ffd7; color: #000;">194</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #d7ffff; color: #000;">195</div>
/// </div>
/// <div style="display: flex;">
///     <div style="flex: 1; display: grid; place-items: center; background: #ff0000; color: #fff;">196</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #ff005f; color: #fff;">197</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #ff0087; color: #fff;">198</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #ff00af; color: #fff;">199</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #ff00d7; color: #fff;">200</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #ff00ff; color: #fff;">201</div>
/// </div>
/// <div style="display: flex;">
///     <div style="flex: 1; display: grid; place-items: center; background: #ff5f00; color: #fff;">202</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #ff5f5f; color: #fff;">203</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #ff5f87; color: #fff;">204</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #ff5faf; color: #fff;">205</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #ff5fd7; color: #fff;">206</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #ff5fff; color: #fff;">207</div>
/// </div>
/// <div style="display: flex;">
///     <div style="flex: 1; display: grid; place-items: center; background: #ff8700; color: #fff;">208</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #ff875f; color: #fff;">209</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #ff8787; color: #fff;">210</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #ff87af; color: #fff;">211</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #ff87d7; color: #fff;">212</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #ff87ff; color: #fff;">213</div>
/// </div>
/// <div style="display: flex;">
///     <div style="flex: 1; display: grid; place-items: center; background: #ffaf00; color: #000;">214</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #ffaf5f; color: #000;">215</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #ffaf87; color: #000;">216</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #ffafaf; color: #000;">217</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #ffafd7; color: #000;">218</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #ffafff; color: #000;">219</div>
/// </div>
/// <div style="display: flex;">
///     <div style="flex: 1; display: grid; place-items: center; background: #ffd700; color: #000;">220</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #ffd75f; color: #000;">221</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #ffd787; color: #000;">222</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #ffd7af; color: #000;">223</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #ffd7d7; color: #000;">224</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #ffd7ff; color: #000;">225</div>
/// </div>
/// <div style="display: flex;">
///     <div style="flex: 1; display: grid; place-items: center; background: #ffff00; color: #000;">226</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #ffff5f; color: #000;">227</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #ffff87; color: #000;">228</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #ffffaf; color: #000;">229</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #ffffd7; color: #000;">230</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #ffffff; color: #000;">231</div>
/// </div>
/// <br>
/// grayscale colors (232..=255)
/// <div style="display: flex;">
///     <div style="flex: 1; display: grid; place-items: center; background: #080808; color: #fff;">232</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #121212; color: #fff;">233</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #1c1c1c; color: #fff;">234</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #262626; color: #fff;">235</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #303030; color: #fff;">236</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #3a3a3a; color: #fff;">237</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #444444; color: #fff;">238</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #4e4e4e; color: #fff;">239</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #585858; color: #fff;">240</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #626262; color: #fff;">241</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #6c6c6c; color: #fff;">242</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #767676; color: #fff;">243</div>
/// </div>
/// <div style="display: flex;">
///     <div style="flex: 1; display: grid; place-items: center; background: #808080; color: #000;">244</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #8a8a8a; color: #000;">245</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #949494; color: #000;">246</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #9e9e9e; color: #000;">247</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #a8a8a8; color: #000;">248</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #b2b2b2; color: #000;">249</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #bcbcbc; color: #000;">250</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #c6c6c6; color: #000;">251</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #d0d0d0; color: #000;">252</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #dadada; color: #000;">253</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #e4e4e4; color: #000;">254</div>
///     <div style="flex: 1; display: grid; place-items: center; background: #eeeeee; color: #000;">255</div>
/// </div>
#[derive(Clone, Copy, Debug)]
pub struct LutColor {
    /// index into the lookup table
    pub index: u8
}

/// get an [`RgbColor`]
pub fn rgb(red: u8, green: u8, blue: u8) -> RgbColor { RgbColor { red, green, blue } }

/// RGB
#[derive(Clone, Copy, Debug)]
pub struct RgbColor {
    /// red channel
    pub red:   u8,
    /// green channel
    pub green: u8,
    /// blue channel
    pub blue:  u8
}

/// different color systems implement this common trait
pub trait Color {
    /// builds a printable ANSI escape
    fn text(&self, colored: Colored) -> String;
}

/// what exactly is being colored? (tells [`Color::text`] what to do)
pub enum Colored {
    /// colored foreground
    Fg,
    /// colored background
    Bg,
    /// colored underline
    Underline
}

impl Color for AnsiColor {
    fn text(&self, colored: Colored) -> String {
        format!(
            "\x1b[{}{}m",
            match colored {
                Colored::Fg        => { if self.bright {  9 } else { 3 } },
                Colored::Bg        => { if self.bright { 10 } else { 4 } },
                Colored::Underline => { if self.bright { 11 } else { 5 } }
            },
            self.number
        )
    }
}

impl Color for LutColor {
    fn text(&self, colored: Colored) -> String {
        format!(
            "\x1b[{}8;5;{}m",
            match colored {
                Colored::Fg        => 3,
                Colored::Bg        => 4,
                Colored::Underline => 5,
            },
            self.index
        )
    }
}

impl Color for RgbColor {
    fn text(&self, colored: Colored) -> String {
        format!(
            "\x1b[{}8;2;{};{};{}m",
            match colored {
                Colored::Fg        => 3,
                Colored::Bg        => 4,
                Colored::Underline => 5,
            },
            self.red,
            self.green,
            self.blue
        )
    }
}

/// builds a printable ANSI escape from color
pub fn fg(color: impl Color) -> String {
    color.text(Colored::Fg)
}

/// builds a printable ANSI escape from color
pub fn bg(color: impl Color) -> String {
    color.text(Colored::Bg)
}

/// builds a printable ANSI escape from color
pub fn underline(color: impl Color) -> String {
    color.text(Colored::Underline)
}

