#[doc = "Register `DCEST_SHIFT` reader"]
pub type R = crate::R<DcestShiftSpec>;
#[doc = "Register `DCEST_SHIFT` writer"]
pub type W = crate::W<DcestShiftSpec>;
#[doc = "Field `DCEST_SHIFT` reader - 3:0\\]
Programmable shift applied to all 6 accumulator outputs. Cannot be bypassed. Scaled accumulator output is shifted by 2^( 2+DCEST_SHIFT). For DCEST_SHIFT = 15 also gives 2^(24) and not 25 (saturate at 24)"]
pub type DcestShiftR = crate::FieldReader;
#[doc = "Field `DCEST_SHIFT` writer - 3:0\\]
Programmable shift applied to all 6 accumulator outputs. Cannot be bypassed. Scaled accumulator output is shifted by 2^( 2+DCEST_SHIFT). For DCEST_SHIFT = 15 also gives 2^(24) and not 25 (saturate at 24)"]
pub type DcestShiftW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader<u32>;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Programmable shift applied to all 6 accumulator outputs. Cannot be bypassed. Scaled accumulator output is shifted by 2^( 2+DCEST_SHIFT). For DCEST_SHIFT = 15 also gives 2^(24) and not 25 (saturate at 24)"]
    #[inline(always)]
    pub fn dcest_shift(&self) -> DcestShiftR {
        DcestShiftR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Programmable shift applied to all 6 accumulator outputs. Cannot be bypassed. Scaled accumulator output is shifted by 2^( 2+DCEST_SHIFT). For DCEST_SHIFT = 15 also gives 2^(24) and not 25 (saturate at 24)"]
    #[inline(always)]
    #[must_use]
    pub fn dcest_shift(&mut self) -> DcestShiftW<DcestShiftSpec> {
        DcestShiftW::new(self, 0)
    }
    #[doc = "Bits 4:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<DcestShiftSpec> {
        Nu1W::new(self, 4)
    }
}
#[doc = "DCEST_SHIFT\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest_shift::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest_shift::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcestShiftSpec;
impl crate::RegisterSpec for DcestShiftSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcest_shift::R`](R) reader structure"]
impl crate::Readable for DcestShiftSpec {}
#[doc = "`write(|w| ..)` method takes [`dcest_shift::W`](W) writer structure"]
impl crate::Writable for DcestShiftSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCEST_SHIFT to value 0"]
impl crate::Resettable for DcestShiftSpec {
    const RESET_VALUE: u32 = 0;
}
