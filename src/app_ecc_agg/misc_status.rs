#[doc = "Register `MISC_STATUS` reader"]
pub type R = crate::R<MiscStatusSpec>;
#[doc = "Register `MISC_STATUS` writer"]
pub type W = crate::W<MiscStatusSpec>;
#[doc = "Field `NUM_RAMS` reader - 10:0\\]
Indicates the number of RAMS serviced by the ECC aggregator - (RO )"]
pub type NumRamsR = crate::FieldReader<u16>;
#[doc = "Field `NUM_RAMS` writer - 10:0\\]
Indicates the number of RAMS serviced by the ECC aggregator - (RO )"]
pub type NumRamsW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `RES3` reader - 31:11\\]
RESERVE FIELD"]
pub type Res3R = crate::FieldReader<u32>;
#[doc = "Field `RES3` writer - 31:11\\]
RESERVE FIELD"]
pub type Res3W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Indicates the number of RAMS serviced by the ECC aggregator - (RO )"]
    #[inline(always)]
    pub fn num_rams(&self) -> NumRamsR {
        NumRamsR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:31 - 31:11\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res3(&self) -> Res3R {
        Res3R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Indicates the number of RAMS serviced by the ECC aggregator - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn num_rams(&mut self) -> NumRamsW<MiscStatusSpec> {
        NumRamsW::new(self, 0)
    }
    #[doc = "Bits 11:31 - 31:11\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res3(&mut self) -> Res3W<MiscStatusSpec> {
        Res3W::new(self, 11)
    }
}
#[doc = "Misc Status\n\nYou can [`read`](crate::Reg::read) this register and get [`misc_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiscStatusSpec;
impl crate::RegisterSpec for MiscStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc_status::R`](R) reader structure"]
impl crate::Readable for MiscStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`misc_status::W`](W) writer structure"]
impl crate::Writable for MiscStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MISC_STATUS to value 0"]
impl crate::Resettable for MiscStatusSpec {
    const RESET_VALUE: u32 = 0;
}
