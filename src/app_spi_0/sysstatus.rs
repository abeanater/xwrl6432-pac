#[doc = "Register `SYSSTATUS` reader"]
pub type R = crate::R<SysstatusSpec>;
#[doc = "Register `SYSSTATUS` writer"]
pub type W = crate::W<SysstatusSpec>;
#[doc = "Field `RESETDONE` reader - 0:0\\]
Internal Reset Monitoring - (RO )"]
pub type ResetdoneR = crate::BitReader;
#[doc = "Field `RESETDONE` writer - 0:0\\]
Internal Reset Monitoring - (RO )"]
pub type ResetdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED_16` reader - 31:1\\]
Reserved for module specific status information Read returns 0 - (RO )"]
pub type Reserved16R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED_16` writer - 31:1\\]
Reserved for module specific status information Read returns 0 - (RO )"]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal Reset Monitoring - (RO )"]
    #[inline(always)]
    pub fn resetdone(&self) -> ResetdoneR {
        ResetdoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved for module specific status information Read returns 0 - (RO )"]
    #[inline(always)]
    pub fn reserved_16(&self) -> Reserved16R {
        Reserved16R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal Reset Monitoring - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn resetdone(&mut self) -> ResetdoneW<SysstatusSpec> {
        ResetdoneW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved for module specific status information Read returns 0 - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn reserved_16(&mut self) -> Reserved16W<SysstatusSpec> {
        Reserved16W::new(self, 1)
    }
}
#[doc = "This register provides status information about the module excluding the interrupt status information\n\nYou can [`read`](crate::Reg::read) this register and get [`sysstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysstatusSpec;
impl crate::RegisterSpec for SysstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysstatus::R`](R) reader structure"]
impl crate::Readable for SysstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`sysstatus::W`](W) writer structure"]
impl crate::Writable for SysstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSSTATUS to value 0"]
impl crate::Resettable for SysstatusSpec {
    const RESET_VALUE: u32 = 0;
}
