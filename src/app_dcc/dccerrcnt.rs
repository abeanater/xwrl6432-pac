#[doc = "Register `DCCERRCNT` reader"]
pub type R = crate::R<DccerrcntSpec>;
#[doc = "Register `DCCERRCNT` writer"]
pub type W = crate::W<DccerrcntSpec>;
#[doc = "Field `ERRCNT` reader - 9:0\\]
Counts the number of errors after the last write to this register or reset. If reached terminal count the count freezes. User needs to clear it."]
pub type ErrcntR = crate::FieldReader<u16>;
#[doc = "Field `ERRCNT` writer - 9:0\\]
Counts the number of errors after the last write to this register or reset. If reached terminal count the count freezes. User needs to clear it."]
pub type ErrcntW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `NU15` reader - 31:10\\]
Reserved"]
pub type Nu15R = crate::FieldReader<u32>;
#[doc = "Field `NU15` writer - 31:10\\]
Reserved"]
pub type Nu15W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Counts the number of errors after the last write to this register or reset. If reached terminal count the count freezes. User needs to clear it."]
    #[inline(always)]
    pub fn errcnt(&self) -> ErrcntR {
        ErrcntR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Reserved"]
    #[inline(always)]
    pub fn nu15(&self) -> Nu15R {
        Nu15R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Counts the number of errors after the last write to this register or reset. If reached terminal count the count freezes. User needs to clear it."]
    #[inline(always)]
    #[must_use]
    pub fn errcnt(&mut self) -> ErrcntW<DccerrcntSpec> {
        ErrcntW::new(self, 0)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu15(&mut self) -> Nu15W<DccerrcntSpec> {
        Nu15W::new(self, 10)
    }
}
#[doc = "Error count register\n\nYou can [`read`](crate::Reg::read) this register and get [`dccerrcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dccerrcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DccerrcntSpec;
impl crate::RegisterSpec for DccerrcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dccerrcnt::R`](R) reader structure"]
impl crate::Readable for DccerrcntSpec {}
#[doc = "`write(|w| ..)` method takes [`dccerrcnt::W`](W) writer structure"]
impl crate::Writable for DccerrcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCCERRCNT to value 0"]
impl crate::Resettable for DccerrcntSpec {
    const RESET_VALUE: u32 = 0;
}
