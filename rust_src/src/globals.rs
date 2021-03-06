#![allow(dead_code)] // https://github.com/rust-lang/rust/issues/38780

/// Based on globals.h, which is auto generated by 'make docfile'.

use lisp::{EmacsInt, LispObject};

extern "C" {
    pub static mut globals: EmacsGlobals;
}

/// EmacsGlobals represents the global state of the editor.
///
/// This has been factored out to a single struct in C Emacs to help
/// with future threading support.
#[repr(C)]
pub struct EmacsGlobals {
    f_Vafter_change_functions: LispObject,
    f_Vafter_init_time: LispObject,
    f_Vafter_insert_file_functions: LispObject,
    f_Vafter_load_alist: LispObject,
    f_Valternate_fontname_alist: LispObject,
    f_Vauto_composition_function: LispObject,
    f_Vauto_composition_mode: LispObject,
    f_Vauto_fill_chars: LispObject,
    f_Vauto_resize_tool_bars: LispObject,
    f_Vauto_save_include_big_deletions: LispObject,
    f_Vauto_save_list_file_name: LispObject,
    f_Vauto_save_timeout: LispObject,
    f_Vauto_save_visited_file_name: LispObject,
    f_Vbefore_change_functions: LispObject,
    f_Vbefore_init_time: LispObject,
    f_Vblink_cursor_alist: LispObject,
    f_Vbuffer_access_fontified_property: LispObject,
    f_Vbuffer_access_fontify_functions: LispObject,
    f_Vbuffer_list_update_hook: LispObject,
    f_Vbuild_files: LispObject,
    f_Vbyte_boolean_vars: LispObject,
    f_Vbyte_code_meter: LispObject,
    f_Vbytecomp_version_regexp: LispObject,
    f_Vcairo_version_string: LispObject,
    f_Vchange_major_mode_hook: LispObject,
    f_Vchar_code_property_alist: LispObject,
    f_Vchar_property_alias_alist: LispObject,
    f_Vchar_script_table: LispObject,
    f_Vchar_width_table: LispObject,
    f_Vcharset_list: LispObject,
    f_Vcharset_map_path: LispObject,
    f_Vcharset_revision_table: LispObject,
    f_Vcode_conversion_map_vector: LispObject,
    f_Vcoding_category_list: LispObject,
    f_Vcoding_system_alist: LispObject,
    f_Vcoding_system_for_read: LispObject,
    f_Vcoding_system_for_write: LispObject,
    f_Vcoding_system_list: LispObject,
    f_Vcombine_after_change_calls: LispObject,
    f_Vcommand_debug_status: LispObject,
    f_Vcommand_error_function: LispObject,
    f_Vcommand_history: LispObject,
    f_Vcommand_line_args: LispObject,
    f_Vcompletion_ignored_extensions: LispObject,
    f_Vcompletion_regexp_list: LispObject,
    f_Vcompose_chars_after_function: LispObject,
    f_Vcomposition_function_table: LispObject,
    f_Vconfigure_info_directory: LispObject,
    f_Vcurrent_iso639_language: LispObject,
    f_Vcurrent_load_list: LispObject,
    f_Vcurrent_prefix_arg: LispObject,
    f_Vdata_directory: LispObject,
    f_Vdbus_compiled_version: LispObject,
    f_Vdbus_debug: LispObject,
    f_Vdbus_message_type_error: LispObject,
    f_Vdbus_message_type_invalid: LispObject,
    f_Vdbus_message_type_method_call: LispObject,
    f_Vdbus_message_type_method_return: LispObject,
    f_Vdbus_message_type_signal: LispObject,
    f_Vdbus_registered_objects_table: LispObject,
    f_Vdbus_runtime_version: LispObject,
    f_Vdeactivate_mark: LispObject,
    f_Vdebug_ignored_errors: LispObject,
    f_Vdebug_on_error: LispObject,
    f_Vdebug_on_event: LispObject,
    f_Vdebug_on_message: LispObject,
    f_Vdebug_on_signal: LispObject,
    f_Vdebugger: LispObject,
    f_Vdefault_file_name_coding_system: LispObject,
    f_Vdefault_frame_alist: LispObject,
    f_Vdefault_frame_scroll_bars: LispObject,
    f_Vdefault_process_coding_system: LispObject,
    f_Vdefault_text_properties: LispObject,
    f_Vdeferred_action_function: LispObject,
    f_Vdeferred_action_list: LispObject,
    f_Vdefine_key_rebound_commands: LispObject,
    f_Vdelayed_warnings_list: LispObject,
    f_Vdelete_frame_functions: LispObject,
    f_Vdelete_terminal_functions: LispObject,
    f_Vdisable_point_adjustment: LispObject,
    f_Vdisplay_pixels_per_inch: LispObject,
    f_Vdoc_directory: LispObject,
    f_Vdoc_file_name: LispObject,
    f_Vdouble_click_time: LispObject,
    f_Vdynamic_library_alist: LispObject,
    f_Vecho_keystrokes: LispObject,
    f_Vemacs_copyright: LispObject,
    f_Vemacs_version: LispObject,
    f_Vemulation_mode_map_alists: LispObject,
    f_Venable_character_translation: LispObject,
    f_Venable_disabled_menus_and_buttons: LispObject,
    f_Veval_buffer_list: LispObject,
    f_Vexec_directory: LispObject,
    f_Vexec_path: LispObject,
    f_Vexec_suffixes: LispObject,
    f_Vexecuting_kbd_macro: LispObject,
    f_Vface_default_stipple: LispObject,
    f_Vface_font_rescale_alist: LispObject,
    f_Vface_ignored_fonts: LispObject,
    f_Vface_new_frame_defaults: LispObject,
    f_Vface_remapping_alist: LispObject,
    f_Vfeatures: LispObject,
    f_Vfile_coding_system_alist: LispObject,
    f_Vfile_name_coding_system: LispObject,
    f_Vfile_name_handler_alist: LispObject,
    f_Vfind_word_boundary_function_table: LispObject,
    f_Vfirst_change_hook: LispObject,
    f_Vfloat_output_format: LispObject,
    f_Vfocus_in_hook: LispObject,
    f_Vfocus_out_hook: LispObject,
    f_Vfont_ccl_encoder_alist: LispObject,
    f_Vfont_encoding_alist: LispObject,
    f_Vfont_encoding_charset_alist: LispObject,
    f_Vfont_log: LispObject,
    f_Vfont_slant_table: LispObject,
    f_Vfont_weight_table: LispObject,
    f_Vfont_width_table: LispObject,
    f_Vfontification_functions: LispObject,
    f_Vfontset_alias_alist: LispObject,
    f_Vframe_alpha_lower_limit: LispObject,
    f_Vframe_title_format: LispObject,
    f_Vfringe_bitmaps: LispObject,
    f_Vfunction_key_map: LispObject,
    f_Vgc_cons_percentage: LispObject,
    f_Vgc_elapsed: LispObject,
    f_Vglobal_disable_point_adjustment: LispObject,
    f_Vglobal_mode_string: LispObject,
    f_Vglyph_table: LispObject,
    f_Vglyphless_char_display: LispObject,
    f_Vgtk_version_string: LispObject,
    f_Vhelp_char: LispObject,
    f_Vhelp_event_list: LispObject,
    f_Vhelp_form: LispObject,
    f_Vhistory_add_new_input: LispObject,
    f_Vhistory_length: LispObject,
    f_Vhourglass_delay: LispObject,
    f_Vhscroll_step: LispObject,
    f_Vicon_title_format: LispObject,
    f_Vignore_relative_composition: LispObject,
    f_Vimage_cache_eviction_delay: LispObject,
    f_Vimage_types: LispObject,
    f_Vinhibit_changing_match_data: LispObject,
    f_Vinhibit_debugger: LispObject,
    f_Vinhibit_field_text_motion: LispObject,
    f_Vinhibit_file_name_handlers: LispObject,
    f_Vinhibit_file_name_operation: LispObject,
    f_Vinhibit_point_motion_hooks: LispObject,
    f_Vinhibit_quit: LispObject,
    f_Vinhibit_read_only: LispObject,
    f_Vinhibit_redisplay: LispObject,
    f_Vinitial_environment: LispObject,
    f_Vinitial_window_system: LispObject,
    f_Vinput_method_function: LispObject,
    f_Vinput_method_previous_message: LispObject,
    f_Vinstallation_directory: LispObject,
    f_Vinternal__top_level_message: LispObject,
    f_Vinternal_interpreter_environment: LispObject,
    f_Vinvocation_directory: LispObject,
    f_Vinvocation_name: LispObject,
    f_Vkbd_macro_termination_hook: LispObject,
    f_Vkey_translation_map: LispObject,
    f_Vkill_buffer_query_functions: LispObject,
    f_Vkill_emacs_hook: LispObject,
    f_Vlast_code_conversion_error: LispObject,
    f_Vlast_coding_system_used: LispObject,
    f_Vlast_event_frame: LispObject,
    f_Vlatin_extra_code_table: LispObject,
    f_Vlexical_binding: LispObject,
    f_Vline_number_display_limit: LispObject,
    f_Vline_prefix: LispObject,
    f_Vload_file_name: LispObject,
    f_Vload_file_rep_suffixes: LispObject,
    f_Vload_history: LispObject,
    f_Vload_path: LispObject,
    f_Vload_read_function: LispObject,
    f_Vload_source_file_function: LispObject,
    f_Vload_suffixes: LispObject,
    f_Vlocale_coding_system: LispObject,
    f_Vlucid_menu_bar_dirty_flag: LispObject,
    f_Vmake_pointer_invisible: LispObject,
    f_Vmark_even_if_inactive: LispObject,
    f_Vmax_image_size: LispObject,
    f_Vmax_mini_window_height: LispObject,
    f_Vmemory_full: LispObject,
    f_Vmemory_signal_data: LispObject,
    f_Vmenu_bar_final_items: LispObject,
    f_Vmenu_bar_mode: LispObject,
    f_Vmenu_bar_update_hook: LispObject,
    f_Vmenu_updating_frame: LispObject,
    f_Vmessage_log_max: LispObject,
    f_Vminibuf_scroll_window: LispObject,
    f_Vminibuffer_completing_file_name: LispObject,
    f_Vminibuffer_completion_confirm: LispObject,
    f_Vminibuffer_completion_predicate: LispObject,
    f_Vminibuffer_completion_table: LispObject,
    f_Vminibuffer_exit_hook: LispObject,
    f_Vminibuffer_help_form: LispObject,
    f_Vminibuffer_history_position: LispObject,
    f_Vminibuffer_history_variable: LispObject,
    f_Vminibuffer_local_map: LispObject,
    f_Vminibuffer_local_ns_map: LispObject,
    f_Vminibuffer_message_timeout: LispObject,
    f_Vminibuffer_prompt_properties: LispObject,
    f_Vminibuffer_setup_hook: LispObject,
    f_Vminor_mode_map_alist: LispObject,
    f_Vminor_mode_overriding_map_alist: LispObject,
    f_Vmodule_file_suffix: LispObject,
    f_Vmost_negative_fixnum: LispObject,
    f_Vmost_positive_fixnum: LispObject,
    f_Vmotif_version_string: LispObject,
    f_Vmouse_autoselect_window: LispObject,
    f_Vmouse_highlight: LispObject,
    f_Vmouse_leave_buffer_hook: LispObject,
    f_Vmouse_position_function: LispObject,
    f_Vnetwork_coding_system_alist: LispObject,
    f_Vnobreak_char_display: LispObject,
    f_Vobarray: LispObject,
    f_Vold_style_backquotes: LispObject,
    f_Voperating_system_release: LispObject,
    f_Votf_script_alist: LispObject,
    f_Vother_window_scroll_buffer: LispObject,
    f_Voverflow_newline_into_fringe: LispObject,
    f_Voverlay_arrow_position: LispObject,
    f_Voverlay_arrow_string: LispObject,
    f_Voverlay_arrow_variable_list: LispObject,
    f_Voverriding_local_map: LispObject,
    f_Voverriding_local_map_menu_flag: LispObject,
    f_Vpath_separator: LispObject,
    f_Vpost_command_hook: LispObject,
    f_Vpost_gc_hook: LispObject,
    f_Vpost_self_insert_hook: LispObject,
    f_Vpre_command_hook: LispObject,
    f_Vpre_redisplay_function: LispObject,
    f_Vprefix_help_command: LispObject,
    f_Vpreloaded_file_list: LispObject,
    f_Vprevious_system_messages_locale: LispObject,
    f_Vprevious_system_time_locale: LispObject,
    f_Vprint_charset_text_property: LispObject,
    f_Vprint_circle: LispObject,
    f_Vprint_continuous_numbering: LispObject,
    f_Vprint_gensym: LispObject,
    f_Vprint_length: LispObject,
    f_Vprint_level: LispObject,
    f_Vprint_number_table: LispObject,
    f_Vprintable_chars: LispObject,
    f_Vprocess_adaptive_read_buffering: LispObject,
    f_Vprocess_coding_system_alist: LispObject,
    f_Vprocess_connection_type: LispObject,
    f_Vprocess_environment: LispObject,
    f_Vpurify_flag: LispObject,
    f_Vquit_flag: LispObject,
    f_Vread_buffer_function: LispObject,
    f_Vread_circle: LispObject,
    f_Vread_expression_history: LispObject,
    f_Vread_hide_char: LispObject,
    f_Vread_symbol_positions_list: LispObject,
    f_Vread_with_symbol_positions: LispObject,
    f_Vreal_this_command: LispObject,
    f_Vrecenter_redisplay: LispObject,
    f_Vredisplay__all_windows_cause: LispObject,
    f_Vredisplay__mode_lines_cause: LispObject,
    f_Vredisplay__variables: LispObject,
    f_Vredisplay_end_trigger_functions: LispObject,
    f_Vreport_emacs_bug_address: LispObject,
    f_Vresize_mini_windows: LispObject,
    f_Vresume_tty_functions: LispObject,
    f_Vring_bell_function: LispObject,
    f_Vsaved_region_selection: LispObject,
    f_Vscalable_fonts_allowed: LispObject,
    f_Vscript_representative_chars: LispObject,
    f_Vscroll_preserve_screen_position: LispObject,
    f_Vsearch_spaces_regexp: LispObject,
    f_Vselect_active_regions: LispObject,
    f_Vselect_safe_coding_system_function: LispObject,
    f_Vselection_converter_alist: LispObject,
    f_Vselection_inhibit_update_commands: LispObject,
    f_Vset_auto_coding_function: LispObject,
    f_Vshared_game_score_directory: LispObject,
    f_Vshell_file_name: LispObject,
    f_Vshow_help_function: LispObject,
    f_Vshow_trailing_whitespace: LispObject,
    f_Vsignal_hook_function: LispObject,
    f_Vsource_directory: LispObject,
    f_Vspecial_event_map: LispObject,
    f_Vstandard_display_table: LispObject,
    f_Vstandard_input: LispObject,
    f_Vstandard_output: LispObject,
    f_Vstandard_translation_table_for_decode: LispObject,
    f_Vstandard_translation_table_for_encode: LispObject,
    f_Vsuspend_tty_functions: LispObject,
    f_Vsystem_configuration: LispObject,
    f_Vsystem_configuration_features: LispObject,
    f_Vsystem_configuration_options: LispObject,
    f_Vsystem_messages_locale: LispObject,
    f_Vsystem_name: LispObject,
    f_Vsystem_time_locale: LispObject,
    f_Vsystem_type: LispObject,
    f_Vtemp_buffer_show_function: LispObject,
    f_Vtemporary_file_directory: LispObject,
    f_Vterminal_frame: LispObject,
    f_Vtext_property_default_nonsticky: LispObject,
    f_Vtext_quoting_style: LispObject,
    f_Vthis_command: LispObject,
    f_Vthis_command_keys_shift_translated: LispObject,
    f_Vthis_original_command: LispObject,
    f_Vthrow_on_input: LispObject,
    f_Vtimer_idle_list: LispObject,
    f_Vtimer_list: LispObject,
    f_Vtool_bar_border: LispObject,
    f_Vtool_bar_button_margin: LispObject,
    f_Vtool_bar_mode: LispObject,
    f_Vtool_bar_separator_image_expression: LispObject,
    f_Vtool_bar_style: LispObject,
    f_Vtop_level: LispObject,
    f_Vtransient_mark_mode: LispObject,
    f_Vtranslation_hash_table_vector: LispObject,
    f_Vtranslation_table_for_input: LispObject,
    f_Vtranslation_table_vector: LispObject,
    f_Vtruncate_partial_width_windows: LispObject,
    f_Vtty_defined_color_alist: LispObject,
    f_Vtty_erase_char: LispObject,
    f_Vundo_outer_limit: LispObject,
    f_Vundo_outer_limit_function: LispObject,
    f_Vunicode_category_table: LispObject,
    f_Vunread_command_events: LispObject,
    f_Vunread_input_method_events: LispObject,
    f_Vunread_post_input_method_events: LispObject,
    f_Vuse_default_ascent: LispObject,
    f_Vuser_full_name: LispObject,
    f_Vuser_init_file: LispObject,
    f_Vuser_login_name: LispObject,
    f_Vuser_real_login_name: LispObject,
    f_Vvalues: LispObject,
    f_Vvertical_centering_font_regexp: LispObject,
    f_Vvoid_text_area_pointer: LispObject,
    f_Vwhere_is_preferred_modifier: LispObject,
    f_Vwindow_combination_limit: LispObject,
    f_Vwindow_combination_resize: LispObject,
    f_Vwindow_configuration_change_hook: LispObject,
    f_Vwindow_persistent_parameters: LispObject,
    f_Vwindow_point_insertion_type: LispObject,
    f_Vwindow_scroll_functions: LispObject,
    f_Vwindow_size_change_functions: LispObject,
    f_Vwindow_system_version: LispObject,
    f_Vwindow_text_change_functions: LispObject,
    f_Vword_combining_categories: LispObject,
    f_Vword_separating_categories: LispObject,
    f_Vwrap_prefix: LispObject,
    f_Vwrite_region_annotate_functions: LispObject,
    f_Vwrite_region_annotations_so_far: LispObject,
    f_Vwrite_region_post_annotation_function: LispObject,
    f_Vx_alt_keysym: LispObject,
    f_Vx_bitmap_file_path: LispObject,
    f_Vx_cursor_fore_pixel: LispObject,
    f_Vx_hourglass_pointer_shape: LispObject,
    f_Vx_hyper_keysym: LispObject,
    f_Vx_keysym_table: LispObject,
    f_Vx_lost_selection_functions: LispObject,
    f_Vx_max_tooltip_size: LispObject,
    f_Vx_meta_keysym: LispObject,
    f_Vx_mode_pointer_shape: LispObject,
    f_Vx_no_window_manager: LispObject,
    f_Vx_nontext_pointer_shape: LispObject,
    f_Vx_pixel_size_width_font_regexp: LispObject,
    f_Vx_pointer_shape: LispObject,
    f_Vx_resource_class: LispObject,
    f_Vx_resource_name: LispObject,
    f_Vx_select_enable_clipboard_manager: LispObject,
    f_Vx_sensitive_text_pointer_shape: LispObject,
    f_Vx_sent_selection_functions: LispObject,
    f_Vx_session_id: LispObject,
    f_Vx_session_previous_id: LispObject,
    f_Vx_super_keysym: LispObject,
    f_Vx_toolkit_scroll_bars: LispObject,
    f_Vx_window_horizontal_drag_shape: LispObject,
    f_Vx_window_vertical_drag_shape: LispObject,
    f_Vxft_settings: LispObject,
    f_do_mouse_tracking: LispObject,
    f_eol_mnemonic_dos: LispObject,
    f_eol_mnemonic_mac: LispObject,
    f_eol_mnemonic_undecided: LispObject,
    f_eol_mnemonic_unix: LispObject,
    f_frame_inhibit_implied_resize: LispObject,
    f_frame_size_history: LispObject,
    f_last_command_event: LispObject,
    f_last_input_event: LispObject,
    f_last_nonmenu_event: LispObject,
    f_menu_prompt_more_char: LispObject,
    f_meta_prefix_char: LispObject,
    f_auto_save_interval: EmacsInt,
    f_baud_rate: EmacsInt,
    f_cons_cells_consed: EmacsInt,
    f_debug_end_pos: EmacsInt,
    f_double_click_fuzz: EmacsInt,
    f_emacs_scroll_step: EmacsInt,
    f_executing_kbd_macro_index: EmacsInt,
    f_extra_keyboard_modifiers: EmacsInt,
    f_floats_consed: EmacsInt,
    f_gc_cons_threshold: EmacsInt,
    f_gcs_done: EmacsInt,
    f_global_gnutls_log_level: EmacsInt,
    f_hscroll_margin: EmacsInt,
    f_imagemagick_render_type: EmacsInt,
    f_intervals_consed: EmacsInt,
    f_line_number_display_limit_width: EmacsInt,
    f_max_lisp_eval_depth: EmacsInt,
    f_max_specpdl_size: EmacsInt,
    f_misc_objects_consed: EmacsInt,
    f_next_screen_context_lines: EmacsInt,
    f_num_input_keys: EmacsInt,
    f_num_nonmacro_input_events: EmacsInt,
    f_overline_margin: EmacsInt,
    f_polling_period: EmacsInt,
    f_profiler_log_size: EmacsInt,
    f_profiler_max_stack_depth: EmacsInt,
    f_pure_bytes_used: EmacsInt,
    f_scroll_conservatively: EmacsInt,
    f_scroll_margin: EmacsInt,
    f_string_chars_consed: EmacsInt,
    f_strings_consed: EmacsInt,
    f_symbols_consed: EmacsInt,
    f_syntax_propertize__done: EmacsInt,
    f_tool_bar_button_relief: EmacsInt,
    f_tool_bar_max_label_size: EmacsInt,
    f_underline_minimum_offset: EmacsInt,
    f_undo_limit: EmacsInt,
    f_undo_strong_limit: EmacsInt,
    f_vector_cells_consed: EmacsInt,
    f_x_selection_timeout: EmacsInt,
    f_Vcomment_end_can_be_escaped: bool,
    f_Vfast_but_imprecise_scrolling: bool,
    f_auto_raise_tool_bar_buttons_p: bool,
    f_auto_window_vscroll_p: bool,
    f_automatic_hscrolling_p: bool,
    f_byte_metering_on: bool,
    f_cannot_suspend: bool,
    f_coding_system_require_warning: bool,
    f_completion_ignore_case: bool,
    f_create_lockfiles: bool,
    f_cross_disabled_images: bool,
    f_cursor_in_echo_area: bool,
    f_debug_on_next_call: bool,
    f_debug_on_quit: bool,
    f_debugger_may_continue: bool,
    f_delete_by_moving_to_trash: bool,
    f_delete_exited_processes: bool,
    f_disable_ascii_optimization: bool,
    f_display_hourglass_p: bool,
    f_enable_recursive_minibuffers: bool,
    f_focus_follows_mouse: bool,
    f_force_load_messages: bool,
    f_frame_resize_pixelwise: bool,
    f_garbage_collection_messages: bool,
    f_highlight_nonselected_windows: bool,
    f_history_delete_duplicates: bool,
    f_indent_tabs_mode: bool,
    f_inherit_process_coding_system: bool,
    f_inhibit_bidi_mirroring: bool,
    f_inhibit_eol_conversion: bool,
    f_inhibit_eval_during_redisplay: bool,
    f_inhibit_free_realized_faces: bool,
    f_inhibit_iso_escape_detection: bool,
    f_inhibit_load_charset_map: bool,
    f_inhibit_menubar_update: bool,
    f_inhibit_message: bool,
    f_inhibit_modification_hooks: bool,
    f_inhibit_null_byte_detection: bool,
    f_inhibit_try_cursor_movement: bool,
    f_inhibit_try_window_id: bool,
    f_inhibit_try_window_reusing: bool,
    f_inhibit_x_resources: bool,
    f_inverse_video: bool,
    f_load_convert_to_unibyte: bool,
    f_load_dangerous_libraries: bool,
    f_load_force_doc_strings: bool,
    f_load_in_progress: bool,
    f_load_prefer_newer: bool,
    f_make_cursor_line_fully_visible_p: bool,
    f_menu_prompting: bool,
    f_message_truncate_lines: bool,
    f_minibuffer_allow_text_properties: bool,
    f_minibuffer_auto_raise: bool,
    f_mode_line_in_non_selected_windows: bool,
    f_multibyte_syntax_as_symbol: bool,
    f_multiple_frames: bool,
    f_no_redraw_on_reenter: bool,
    f_noninteractive1: bool,
    f_open_paren_in_column_0_is_defun_start: bool,
    f_parse_sexp_ignore_comments: bool,
    f_parse_sexp_lookup_properties: bool,
    f_print_escape_multibyte: bool,
    f_print_escape_newlines: bool,
    f_print_escape_nonascii: bool,
    f_print_quoted: bool,
    f_read_buffer_completion_ignore_case: bool,
    f_redisplay_dont_pause: bool,
    f_scroll_bar_adjust_thumb_portion_p: bool,
    f_system_uses_terminfo: bool,
    f_text_quoting_flag: bool,
    f_undo_inhibit_record_point: bool,
    f_unibyte_display_via_language_environment: bool,
    f_use_dialog_box: bool,
    f_use_file_dialog: bool,
    f_use_system_font: bool,
    f_visible_bell: bool,
    f_visible_cursor: bool,
    f_window_resize_pixelwise: bool,
    f_words_include_escapes: bool,
    f_write_region_inhibit_fsync: bool,
    f_x_frame_normalize_before_maximize: bool,
    f_x_gtk_file_dialog_help_text: bool,
    f_x_gtk_show_hidden_files: bool,
    f_x_gtk_use_old_file_dialog: bool,
    f_x_gtk_use_system_tooltips: bool,
    f_x_mouse_click_focus_ignore_position: bool,
    f_x_stretch_cursor_p: bool,
    f_x_underline_at_descent_line: bool,
    f_x_use_underline_position_properties: bool,
}
