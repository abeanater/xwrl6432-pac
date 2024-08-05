#[doc = "Register `APPSS_IPC_RFS` reader"]
pub type R = crate::R<AppssIpcRfsSpec>;
#[doc = "Register `APPSS_IPC_RFS` writer"]
pub type W = crate::W<AppssIpcRfsSpec>;
#[doc = "Field `host_intr` reader - 3:0\\]
Write_pulse bit field: Writing 1'b1 to each bit will trigger HOST_INTR &lt;0-3> respectively to CM3."]
pub type HostIntrR = crate::FieldReader;
#[doc = "Field `host_intr` writer - 3:0\\]
Write_pulse bit field: Writing 1'b1 to each bit will trigger HOST_INTR &lt;0-3> respectively to CM3."]
pub type HostIntrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `command` reader - 31:4\\]
Used by software to communicate commands and response. It is 7-bits per interrupt."]
pub type CommandR = crate::FieldReader<u32>;
#[doc = "Field `command` writer - 31:4\\]
Used by software to communicate commands and response. It is 7-bits per interrupt."]
pub type CommandW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Write_pulse bit field: Writing 1'b1 to each bit will trigger HOST_INTR &lt;0-3> respectively to CM3."]
    #[inline(always)]
    pub fn host_intr(&self) -> HostIntrR {
        HostIntrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Used by software to communicate commands and response. It is 7-bits per interrupt."]
    #[inline(always)]
    pub fn command(&self) -> CommandR {
        CommandR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Write_pulse bit field: Writing 1'b1 to each bit will trigger HOST_INTR &lt;0-3> respectively to CM3."]
    #[inline(always)]
    #[must_use]
    pub fn host_intr(&mut self) -> HostIntrW<AppssIpcRfsSpec> {
        HostIntrW::new(self, 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Used by software to communicate commands and response. It is 7-bits per interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn command(&mut self) -> CommandW<AppssIpcRfsSpec> {
        CommandW::new(self, 4)
    }
}
#[doc = "APPSS_IPC_RFS\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_ipc_rfs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_ipc_rfs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssIpcRfsSpec;
impl crate::RegisterSpec for AppssIpcRfsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_ipc_rfs::R`](R) reader structure"]
impl crate::Readable for AppssIpcRfsSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_ipc_rfs::W`](W) writer structure"]
impl crate::Writable for AppssIpcRfsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_IPC_RFS to value 0"]
impl crate::Resettable for AppssIpcRfsSpec {
    const RESET_VALUE: u32 = 0;
}
