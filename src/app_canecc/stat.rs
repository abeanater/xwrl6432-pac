#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "Field `NUM_RAMS` reader - 10:0\\]
Indicates the number of RAMS serviced by the ECC aggregator"]
pub type NumRamsR = crate::FieldReader<u16>;
#[doc = "Field `NUM_RAMS` writer - 10:0\\]
Indicates the number of RAMS serviced by the ECC aggregator"]
pub type NumRamsW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `NU2` reader - 31:11\\]
Reserved"]
pub type Nu2R = crate::FieldReader<u32>;
#[doc = "Field `NU2` writer - 31:11\\]
Reserved"]
pub type Nu2W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Indicates the number of RAMS serviced by the ECC aggregator"]
    #[inline(always)]
    pub fn num_rams(&self) -> NumRamsR {
        NumRamsR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Reserved"]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Indicates the number of RAMS serviced by the ECC aggregator"]
    #[inline(always)]
    #[must_use]
    pub fn num_rams(&mut self) -> NumRamsW<StatSpec> {
        NumRamsW::new(self, 0)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<StatSpec> {
        Nu2W::new(self, 11)
    }
}
#[doc = "Misc Status\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for StatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0;
}
