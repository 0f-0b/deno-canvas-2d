import { core } from "ext:core/mod.js";

export const {
  op_canvas_2d_parse_matrix,
  op_canvas_2d_encode_png,
  op_canvas_2d_decode_image,
  op_canvas_2d_state_new,
  op_canvas_2d_state_width,
  op_canvas_2d_state_set_width,
  op_canvas_2d_state_height,
  op_canvas_2d_state_set_height,
  op_canvas_2d_state_save,
  op_canvas_2d_state_restore,
  op_canvas_2d_state_reset,
  op_canvas_2d_state_clear,
  op_canvas_2d_state_line_width,
  op_canvas_2d_state_set_line_width,
  op_canvas_2d_state_line_cap,
  op_canvas_2d_state_set_line_cap,
  op_canvas_2d_state_line_join,
  op_canvas_2d_state_set_line_join,
  op_canvas_2d_state_miter_limit,
  op_canvas_2d_state_set_miter_limit,
  op_canvas_2d_state_dash_list,
  op_canvas_2d_state_set_dash_list,
  op_canvas_2d_state_line_dash_offset,
  op_canvas_2d_state_set_line_dash_offset,
  op_canvas_2d_state_font,
  op_canvas_2d_state_set_font,
  op_canvas_2d_state_text_align,
  op_canvas_2d_state_set_text_align,
  op_canvas_2d_state_text_baseline,
  op_canvas_2d_state_set_text_baseline,
  op_canvas_2d_state_direction,
  op_canvas_2d_state_set_direction,
  op_canvas_2d_state_letter_spacing,
  op_canvas_2d_state_set_letter_spacing,
  op_canvas_2d_state_word_spacing,
  op_canvas_2d_state_set_word_spacing,
  op_canvas_2d_state_font_kerning,
  op_canvas_2d_state_set_font_kerning,
  op_canvas_2d_state_font_stretch,
  op_canvas_2d_state_set_font_stretch,
  op_canvas_2d_state_font_variant_caps,
  op_canvas_2d_state_set_font_variant_caps,
  op_canvas_2d_state_text_rendering,
  op_canvas_2d_state_set_text_rendering,
  op_canvas_2d_state_scale,
  op_canvas_2d_state_rotate,
  op_canvas_2d_state_translate,
  op_canvas_2d_state_transform,
  op_canvas_2d_state_get_transform,
  op_canvas_2d_state_set_transform,
  op_canvas_2d_state_reset_transform,
  op_canvas_2d_state_fill_style,
  op_canvas_2d_state_set_fill_style_color,
  op_canvas_2d_state_set_fill_style_gradient,
  op_canvas_2d_state_set_fill_style_pattern,
  op_canvas_2d_state_stroke_style,
  op_canvas_2d_state_set_stroke_style_color,
  op_canvas_2d_state_set_stroke_style_gradient,
  op_canvas_2d_state_set_stroke_style_pattern,
  op_canvas_2d_state_clear_rect,
  op_canvas_2d_state_fill_rect,
  op_canvas_2d_state_stroke_rect,
  op_canvas_2d_state_fill_text,
  op_canvas_2d_state_stroke_text,
  op_canvas_2d_state_measure_text,
  op_canvas_2d_state_fill,
  op_canvas_2d_state_stroke,
  op_canvas_2d_state_clip,
  op_canvas_2d_state_is_point_in_path,
  op_canvas_2d_state_is_point_in_stroke,
  op_canvas_2d_state_draw_image,
  op_canvas_2d_state_get_image_data,
  op_canvas_2d_state_put_image_data,
  op_canvas_2d_state_global_alpha,
  op_canvas_2d_state_set_global_alpha,
  op_canvas_2d_state_global_composite_operation,
  op_canvas_2d_state_set_global_composite_operation,
  op_canvas_2d_state_image_smoothing_enabled,
  op_canvas_2d_state_set_image_smoothing_enabled,
  op_canvas_2d_state_image_smoothing_quality,
  op_canvas_2d_state_set_image_smoothing_quality,
  op_canvas_2d_state_shadow_color,
  op_canvas_2d_state_set_shadow_color,
  op_canvas_2d_state_shadow_offset_x,
  op_canvas_2d_state_set_shadow_offset_x,
  op_canvas_2d_state_shadow_offset_y,
  op_canvas_2d_state_set_shadow_offset_y,
  op_canvas_2d_state_shadow_blur,
  op_canvas_2d_state_set_shadow_blur,
  op_canvas_2d_state_set_filter,
  op_canvas_2d_gradient_new_linear,
  op_canvas_2d_gradient_new_radial,
  op_canvas_2d_gradient_new_conic,
  op_canvas_2d_gradient_add_color_stop,
  op_canvas_2d_pattern_new,
  op_canvas_2d_pattern_set_transform,
  op_canvas_2d_path_new,
  op_canvas_2d_path_clone,
  op_canvas_2d_path_clear,
  op_canvas_2d_path_extend,
  op_canvas_2d_path_ensure_subpath,
  op_canvas_2d_path_move_to,
  op_canvas_2d_path_line_to,
  op_canvas_2d_path_quad_to,
  op_canvas_2d_path_cubic_to,
  op_canvas_2d_path_arc_to,
  op_canvas_2d_path_ellipse,
  op_canvas_2d_path_rect,
  op_canvas_2d_path_round_rect,
  op_canvas_2d_path_close,
  op_canvas_2d_image_bitmap_from_canvas_state,
  op_canvas_2d_image_bitmap_from_canvas_state_crop,
  op_canvas_2d_image_bitmap_from_image_data_crop_and_resize,
  op_canvas_2d_image_bitmap_empty,
  op_canvas_2d_image_bitmap_empty_resize,
  op_canvas_2d_image_bitmap_width,
  op_canvas_2d_image_bitmap_height,
  op_canvas_2d_image_bitmap_clone,
  op_canvas_2d_image_bitmap_crop,
  op_canvas_2d_image_bitmap_resize,
  op_canvas_2d_image_bitmap_close,
} = core.ensureFastOps();
