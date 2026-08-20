// auto-generated DO NOT EDIT

#pragma once

#include <algorithm>
#include <array>
#include <functional>
#include <limits>
#include <mutex>
#include <rclcpp/node.hpp>
#include <rclcpp_lifecycle/lifecycle_node.hpp>
#include <rclcpp/logger.hpp>
#include <set>
#include <sstream>
#include <string>
#include <type_traits>
#include <utility>
#include <vector>

#include <fmt/core.h>
#include <fmt/format.h>
#include <fmt/ranges.h>

// silence deprecation warnings for parameter_traits, needed for backwards compatibility
#define SILENCE_DEPRECATION_WARNINGS
#include <parameter_traits/parameter_traits.hpp>
#undef SILENCE_DEPRECATION_WARNINGS

#include <rsl/static_string.hpp>
#include <rsl/static_vector.hpp>
#include <rsl/parameter_validators.hpp>



namespace spot_pose_broadcaster {

// Use validators from RSL
using rsl::unique;
using rsl::subset_of;
using rsl::fixed_size;
using rsl::size_gt;
using rsl::size_lt;
using rsl::not_empty;
using rsl::element_bounds;
using rsl::lower_element_bounds;
using rsl::upper_element_bounds;
using rsl::bounds;
using rsl::lt;
using rsl::gt;
using rsl::lt_eq;
using rsl::gt_eq;
using rsl::one_of;
using rsl::to_parameter_result_msg;

// temporarily needed for backwards compatibility for custom validators
using namespace parameter_traits;

template <typename T>
[[nodiscard]] auto to_parameter_value(T value) {
    return rclcpp::ParameterValue(value);
}

template <size_t capacity>
[[nodiscard]] auto to_parameter_value(rsl::StaticString<capacity> const& value) {
    return rclcpp::ParameterValue(rsl::to_string(value));
}

template <typename T, size_t capacity>
[[nodiscard]] auto to_parameter_value(rsl::StaticVector<T, capacity> const& value) {
    return rclcpp::ParameterValue(rsl::to_vector(value));
}
    struct Params {
        bool use_namespace_as_prefix = true;
        std::string vision_t_body_sensor = "vision_t_body";
        std::string odom_t_body_sensor = "odom_t_body";
        std::string body_frame_name = "body";
        std::string vision_frame_name = "low_level/vision";
        std::string odom_frame_name = "low_level/odom";
        // for detecting if the parameter struct has been updated
        rclcpp::Time __stamp;
    };
    struct StackParams {
        bool use_namespace_as_prefix = true;
    };

  class ParamListener{
  public:
    // throws rclcpp::exceptions::InvalidParameterValueException on initialization if invalid parameter are loaded
    template <typename NodeT>
    ParamListener(NodeT node, std::string const& prefix = "")
    : ParamListener(node->get_node_parameters_interface(), node->get_logger(), prefix) {}

    ParamListener(const std::shared_ptr<rclcpp::node_interfaces::NodeParametersInterface>& parameters_interface,
                  std::string const& prefix = "")
    : ParamListener(parameters_interface, rclcpp::get_logger("spot_pose_broadcaster"), prefix) {
      RCLCPP_DEBUG(logger_, "ParameterListener: Not using node logger, recommend using other constructors to use a node logger");
    }

    ParamListener(const std::shared_ptr<rclcpp::node_interfaces::NodeParametersInterface>& parameters_interface,
                  rclcpp::Logger logger, std::string const& prefix = "")
    : prefix_{prefix},
      logger_{std::move(logger)} {
      if (!prefix_.empty() && prefix_.back() != '.') {
        prefix_ += ".";
      }

      parameters_interface_ = parameters_interface;
      declare_params();
      auto update_param_cb = [this](const std::vector<rclcpp::Parameter> &parameters){return this->update(parameters);};
      handle_ = parameters_interface_->add_on_set_parameters_callback(update_param_cb);
      clock_ = rclcpp::Clock();
    }

    Params get_params() const{
      std::lock_guard<std::mutex> lock(mutex_);
      return params_;
    }

    /**
     * @brief Tries to update the parsed Params object
     * @param params_in The Params object to update
     * @return true if the Params object was updated, false if it was already up to date or the mutex could not be locked
     * @note This function tries to lock the mutex without blocking, so it can be used in a RT loop
     */
    bool try_update_params(Params & params_in) const {
      std::unique_lock<std::mutex> lock(mutex_, std::try_to_lock);
      if (lock.owns_lock()) {
        if (const bool is_old = params_in.__stamp != params_.__stamp; is_old) {
          params_in = params_;
          return true;
        }
      }
      return false;
    }

    /**
     * @brief Tries to get the current Params object
     * @param params_in The Params object to fill with the current parameters
     * @return true if mutex can be locked, false if mutex could not be locked
     * @note The parameters are only filled, when the mutex can be locked and the params timestamp is different
     * @note This function tries to lock the mutex without blocking, so it can be used in a RT loop
     */
    bool try_get_params(Params & params_in) const {
      if (mutex_.try_lock()) {
        if (const bool is_old = params_in.__stamp != params_.__stamp; is_old) {
          params_in = params_;
        }
        mutex_.unlock();
        return true;
      }
      return false;
    }

    bool is_old(Params const& other) const {
      std::lock_guard<std::mutex> lock(mutex_);
      return params_.__stamp != other.__stamp;
    }

    StackParams get_stack_params() {
      Params params = get_params();
      StackParams output;
      output.use_namespace_as_prefix = params.use_namespace_as_prefix;

      return output;
    }

    void refresh_dynamic_parameters() {
      auto updated_params = get_params();
      // TODO remove any destroyed dynamic parameters

      // declare any new dynamic parameters
      rclcpp::Parameter param;

    }

    rcl_interfaces::msg::SetParametersResult update(const std::vector<rclcpp::Parameter> &parameters) {
      auto updated_params = get_params();

      for (const auto &param: parameters) {
        if (param.get_name() == (prefix_ + "use_namespace_as_prefix")) {
            updated_params.use_namespace_as_prefix = param.as_bool();
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "vision_t_body_sensor")) {
            if(auto validation_result = not_empty<std::string>(param);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.vision_t_body_sensor = param.as_string();
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "odom_t_body_sensor")) {
            if(auto validation_result = not_empty<std::string>(param);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.odom_t_body_sensor = param.as_string();
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "body_frame_name")) {
            if(auto validation_result = not_empty<std::string>(param);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.body_frame_name = param.as_string();
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "vision_frame_name")) {
            if(auto validation_result = not_empty<std::string>(param);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.vision_frame_name = param.as_string();
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "odom_frame_name")) {
            if(auto validation_result = not_empty<std::string>(param);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.odom_frame_name = param.as_string();
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
      }

      updated_params.__stamp = clock_.now();
      update_internal_params(updated_params);
      if (user_callback_) {
         user_callback_(updated_params);
      }
      return rsl::to_parameter_result_msg({});
    }

    void declare_params(){
      auto updated_params = get_params();
      // declare all parameters and give default values to non-required ones
      if (!parameters_interface_->has_parameter(prefix_ + "use_namespace_as_prefix")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "Flag to determine if the prefix of the sensor name and TF frames should be picked up from the namespace of the controller.";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.use_namespace_as_prefix);
          parameters_interface_->declare_parameter(prefix_ + "use_namespace_as_prefix", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "vision_t_body_sensor")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "Sensor name containing the transform from the low level vision estimate to body. The state interface names are: <vision_t_body_sensor>/position.x, ..., <vision_t_body_sensor>/position.z, <vision_t_body_sensor>/orientation.x, ..., <vision_t_body_sensor>/orientation.w";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.vision_t_body_sensor);
          parameters_interface_->declare_parameter(prefix_ + "vision_t_body_sensor", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "odom_t_body_sensor")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "Sensor name containing the transform from the low level odom estimate to body. The state interface names are: <odom_t_body_sensor>/position.x, ..., <odom_t_body_sensor>/position.z, <odom_t_body_sensor>/orientation.x, ..., <odom_t_body_sensor>/orientation.w";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.odom_t_body_sensor);
          parameters_interface_->declare_parameter(prefix_ + "odom_t_body_sensor", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "body_frame_name")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "TF frame name for the body";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.body_frame_name);
          parameters_interface_->declare_parameter(prefix_ + "body_frame_name", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "vision_frame_name")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "TF frame name for the low level estimate of the vision frame";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.vision_frame_name);
          parameters_interface_->declare_parameter(prefix_ + "vision_frame_name", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "odom_frame_name")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "TF frame name for the low level estimate of the odom frame";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.odom_frame_name);
          parameters_interface_->declare_parameter(prefix_ + "odom_frame_name", parameter, descriptor);
      }
      // get parameters and fill struct fields
      rclcpp::Parameter param;
      param = parameters_interface_->get_parameter(prefix_ + "use_namespace_as_prefix");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "use_namespace_as_prefix") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.use_namespace_as_prefix = param.as_bool();
      param = parameters_interface_->get_parameter(prefix_ + "vision_t_body_sensor");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "vision_t_body_sensor") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = not_empty<std::string>(param);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'vision_t_body_sensor': {}", validation_result.error()));
      }
      updated_params.vision_t_body_sensor = param.as_string();
      param = parameters_interface_->get_parameter(prefix_ + "odom_t_body_sensor");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "odom_t_body_sensor") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = not_empty<std::string>(param);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'odom_t_body_sensor': {}", validation_result.error()));
      }
      updated_params.odom_t_body_sensor = param.as_string();
      param = parameters_interface_->get_parameter(prefix_ + "body_frame_name");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "body_frame_name") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = not_empty<std::string>(param);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'body_frame_name': {}", validation_result.error()));
      }
      updated_params.body_frame_name = param.as_string();
      param = parameters_interface_->get_parameter(prefix_ + "vision_frame_name");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "vision_frame_name") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = not_empty<std::string>(param);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'vision_frame_name': {}", validation_result.error()));
      }
      updated_params.vision_frame_name = param.as_string();
      param = parameters_interface_->get_parameter(prefix_ + "odom_frame_name");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "odom_frame_name") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = not_empty<std::string>(param);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'odom_frame_name': {}", validation_result.error()));
      }
      updated_params.odom_frame_name = param.as_string();


      updated_params.__stamp = clock_.now();
      update_internal_params(updated_params);
    }

    using userParameterUpdateCB = std::function<void(const Params&)>;
    void setUserCallback(const userParameterUpdateCB& callback){
      user_callback_ = callback;
    }

    void clearUserCallback(){
      user_callback_ = {};
    }

    private:
      void update_internal_params(Params updated_params) {
        std::lock_guard<std::mutex> lock(mutex_);
        params_ = std::move(updated_params);
      }

      std::string prefix_;
      Params params_;
      rclcpp::Clock clock_;
      std::shared_ptr<rclcpp::node_interfaces::OnSetParametersCallbackHandle> handle_;
      std::shared_ptr<rclcpp::node_interfaces::NodeParametersInterface> parameters_interface_;
      userParameterUpdateCB user_callback_;

      rclcpp::Logger logger_;
      std::mutex mutable mutex_;
  };

} // namespace spot_pose_broadcaster
