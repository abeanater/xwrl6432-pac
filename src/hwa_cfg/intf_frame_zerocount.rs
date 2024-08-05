#[doc = "Register `INTF_FRAME_ZEROCOUNT` reader"]
pub type R = crate::R<IntfFrameZerocountSpec>;
#[doc = "Register `INTF_FRAME_ZEROCOUNT` writer"]
pub type W = crate::W<IntfFrameZerocountSpec>;
#[doc = "Field `INTF_FRAME_ZEROCOUNT` reader - 19:0\\]
Number of samples that exceeded the threshold in a frame"]
pub type IntfFrameZerocountR = crate::FieldReader<u32>;
#[doc = "Field `INTF_FRAME_ZEROCOUNT` writer - 19:0\\]
Number of samples that exceeded the threshold in a frame"]
pub type IntfFrameZerocountW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader<u16>;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
Number of samples that exceeded the threshold in a frame"]
    #[inline(always)]
    pub fn intf_frame_zerocount(&self) -> IntfFrameZerocountR {
        IntfFrameZerocountR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
Number of samples that exceeded the threshold in a frame"]
    #[inline(always)]
    #[must_use]
    pub fn intf_frame_zerocount(&mut self) -> IntfFrameZerocountW<IntfFrameZerocountSpec> {
        IntfFrameZerocountW::new(self, 0)
    }
    #[doc = "Bits 20:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<IntfFrameZerocountSpec> {
        Nu1W::new(self, 20)
    }
}
#[doc = "INTF_FRAME_ZEROCOUNT\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_frame_zerocount::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_frame_zerocount::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfFrameZerocountSpec;
impl crate::RegisterSpec for IntfFrameZerocountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_frame_zerocount::R`](R) reader structure"]
impl crate::Readable for IntfFrameZerocountSpec {}
#[doc = "`write(|w| ..)` method takes [`intf_frame_zerocount::W`](W) writer structure"]
impl crate::Writable for IntfFrameZerocountSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_FRAME_ZEROCOUNT to value 0"]
impl crate::Resettable for IntfFrameZerocountSpec {
    const RESET_VALUE: u32 = 0;
}
