using AppointmentScheduling.Services;
using Microsoft.AspNetCore.Http;
using Microsoft.AspNetCore.Mvc;
using System.Security.Claims;

namespace AppointmentScheduling.Controllers
{
    [Route("API/Appointment")]
    [ApiController]
    public class AppointmentAPIController : Controller
    {
        private readonly IAppointmentService _appointment_service;
        private readonly IHttpContextAccessor _http_context_accessor;
        private readonly string login_user_id;
        private readonly string role;

        public AppointmentAPIController(IAppointmentService appointment_service, 
                                        IHttpContextAccessor http_context_accessor)
        {
            _appointment_service = appointment_service;
            _http_context_accessor = http_context_accessor;
            login_user_id = _http_context_accessor.HttpContext.User.FindFirstValue(ClaimTypes.NameIdentifier);
            role = _http_context_accessor.HttpContext.User.FindFirstValue(ClaimTypes.Role);
        }

        [HttpPost]
        [Route("SaveCalendarData")]
        public IActionResult SaveCalendarData(AppointmentViewModel data)
        {
            CommonResponse<int> common_response = new CommonResponse<int>();
            try
            {
                common_response.Status = _appointment_service.AddUpdate(data).Result;

                switch (common_response.Status)
                {
                    case 1:
                        common_response.Message = Helper.Appointment_updated;
                        break;
                    case 2:
                        common_response.Message = Helper.Appointment_added;
                        break;
                    default:
                        common_response.Message = Helper.Something_went_wrong;
                        break;
                }
            }
            catch (Exception e)
            {
                common_response.Message = e.Message;
                common_response.Status = Helper.Failure_code;
            }

            return Ok(common_response);
        }
    }
}
