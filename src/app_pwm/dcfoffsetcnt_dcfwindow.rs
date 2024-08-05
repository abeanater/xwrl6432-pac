#[doc = "Register `DCFOFFSETCNT_DCFWINDOW` reader"]
pub type R = crate::R<DcfoffsetcntDcfwindowSpec>;
#[doc = "Register `DCFOFFSETCNT_DCFWINDOW` writer"]
pub type W = crate::W<DcfoffsetcntDcfwindowSpec>;
#[doc = "Field `DCFOFFSETCNT_OFFSETCNT` reader - 15:0\\]
Blanking Offset Counter These 16-bits are read only and indicate the current value of the offset counter. The counter counts down to zero and then stops until it is re-loaded on the next period or zero event as defined by the DCFCTL\\[PULSESEL\\]
bit. The offset counter is not affected by the free/soft emulation bits. That is, it will always continue to count down if the device is halted by a emulation stop."]
pub type DcfoffsetcntOffsetcntR = crate::FieldReader<u16>;
#[doc = "Field `DCFOFFSETCNT_OFFSETCNT` writer - 15:0\\]
Blanking Offset Counter These 16-bits are read only and indicate the current value of the offset counter. The counter counts down to zero and then stops until it is re-loaded on the next period or zero event as defined by the DCFCTL\\[PULSESEL\\]
bit. The offset counter is not affected by the free/soft emulation bits. That is, it will always continue to count down if the device is halted by a emulation stop."]
pub type DcfoffsetcntOffsetcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DCFWINDOW_WINDOW` reader - 23:16\\]
Blanking Window Width 0 No blanking window is generated. 1h-FFh Specifies the width of the blanking window in TBCLK cycles. The blanking window begins when the offset counter expires. When this occurs, the window counter is loaded and begins to count down. If the blanking window is currently active and the offset counter expires, the blanking window counter is restarted. The blanking window can cross a PWM period boundary."]
pub type DcfwindowWindowR = crate::FieldReader;
#[doc = "Field `DCFWINDOW_WINDOW` writer - 23:16\\]
Blanking Window Width 0 No blanking window is generated. 1h-FFh Specifies the width of the blanking window in TBCLK cycles. The blanking window begins when the offset counter expires. When this occurs, the window counter is loaded and begins to count down. If the blanking window is currently active and the offset counter expires, the blanking window counter is restarted. The blanking window can cross a PWM period boundary."]
pub type DcfwindowWindowW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Reserved1` reader - 31:24\\]
Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - 31:24\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Blanking Offset Counter These 16-bits are read only and indicate the current value of the offset counter. The counter counts down to zero and then stops until it is re-loaded on the next period or zero event as defined by the DCFCTL\\[PULSESEL\\]
bit. The offset counter is not affected by the free/soft emulation bits. That is, it will always continue to count down if the device is halted by a emulation stop."]
    #[inline(always)]
    pub fn dcfoffsetcnt_offsetcnt(&self) -> DcfoffsetcntOffsetcntR {
        DcfoffsetcntOffsetcntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Blanking Window Width 0 No blanking window is generated. 1h-FFh Specifies the width of the blanking window in TBCLK cycles. The blanking window begins when the offset counter expires. When this occurs, the window counter is loaded and begins to count down. If the blanking window is currently active and the offset counter expires, the blanking window counter is restarted. The blanking window can cross a PWM period boundary."]
    #[inline(always)]
    pub fn dcfwindow_window(&self) -> DcfwindowWindowR {
        DcfwindowWindowR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Blanking Offset Counter These 16-bits are read only and indicate the current value of the offset counter. The counter counts down to zero and then stops until it is re-loaded on the next period or zero event as defined by the DCFCTL\\[PULSESEL\\]
bit. The offset counter is not affected by the free/soft emulation bits. That is, it will always continue to count down if the device is halted by a emulation stop."]
    #[inline(always)]
    #[must_use]
    pub fn dcfoffsetcnt_offsetcnt(&mut self) -> DcfoffsetcntOffsetcntW<DcfoffsetcntDcfwindowSpec> {
        DcfoffsetcntOffsetcntW::new(self, 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Blanking Window Width 0 No blanking window is generated. 1h-FFh Specifies the width of the blanking window in TBCLK cycles. The blanking window begins when the offset counter expires. When this occurs, the window counter is loaded and begins to count down. If the blanking window is currently active and the offset counter expires, the blanking window counter is restarted. The blanking window can cross a PWM period boundary."]
    #[inline(always)]
    #[must_use]
    pub fn dcfwindow_window(&mut self) -> DcfwindowWindowW<DcfoffsetcntDcfwindowSpec> {
        DcfwindowWindowW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<DcfoffsetcntDcfwindowSpec> {
        Reserved1W::new(self, 24)
    }
}
#[doc = "Digital Compare Filter Offset Counter Register/ Digital Compare Filter Window Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcfoffsetcnt_dcfwindow::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcfoffsetcnt_dcfwindow::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcfoffsetcntDcfwindowSpec;
impl crate::RegisterSpec for DcfoffsetcntDcfwindowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcfoffsetcnt_dcfwindow::R`](R) reader structure"]
impl crate::Readable for DcfoffsetcntDcfwindowSpec {}
#[doc = "`write(|w| ..)` method takes [`dcfoffsetcnt_dcfwindow::W`](W) writer structure"]
impl crate::Writable for DcfoffsetcntDcfwindowSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCFOFFSETCNT_DCFWINDOW to value 0"]
impl crate::Resettable for DcfoffsetcntDcfwindowSpec {
    const RESET_VALUE: u32 = 0;
}
